use db;
use provider::github::get_github_user;
use diesel::result::Error as DieselErr;
use diesel;
use diesel::pg::PgConnection;
use oauth;
use errors::Error;
use hydra;
use multi_reactor_drifting;
use db::users::User as DBUser;
use db::groups::NewGroup;
use multi_reactor_drifting::Handle;
use futures::Future;
use rocket;
use rocket::http::hyper::header::Bearer as BearerAuth;
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUser {
    pub provider: oauth::Provider,
    pub provider_id: i32,
    pub provider_name: String,
    pub access_token: String,
}

// User is a komadori user.
// Its serialization is returned by the api, so it should be considered the public
// representation of a user and not contain internal details like database id.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub role: Option<String>,
    pub email: String,

    pub groups: Vec<GroupResp>,

    pub auth_metadata: AuthMetadata,

    #[serde(skip)]
    _dbuser: Option<DBUser>,
}

// Custom PartialEq to avoid comparing internal fields that really don't matter
impl PartialEq for User {
    fn eq(&self, rhs: &Self) -> bool {
        return self.uuid == rhs.uuid &&
            self.username == rhs.username &&
            self.role == rhs.role &&
            self.email == rhs.email &&
            self.groups == rhs.groups &&
            self.auth_metadata == rhs.auth_metadata
    }
}

impl User {
    pub fn new(user: DBUser, conn: &PgConnection) -> Result<User, Error> {
        let groups = user.groups(conn)
            .map_err(|e| {
                Error::server_error(format!("error getting groups: {}", e))
            })?;

        // TODO: more oauth providers will break this
        let github_account = {
            user.github_account(conn)
                .map_err(|e| {
                    Error::server_error(format!("error getting accounts: {}", e))
                })
                .map(|c| {
                    GithubAuthMetadata{
                        username: c.username.unwrap_or("".to_string()),
                    }
                })
            .map_err(|e| format!("{:?}", e))
        };

        Ok(User {
            uuid: user.uuid.simple().to_string(),
            username: user.username.clone(),
            role: user.role.clone(),
            email: user.email.clone(),
            groups: groups.iter().map(|g| g.into()).collect(),
            auth_metadata: AuthMetadata{
                github: Some(github_account),
            },
            _dbuser: Some(user),
        })
    }

    pub fn from_uuid(uuid: Uuid, conn: &PgConnection) -> Result<Self, Error> {
        let dbuser = match DBUser::from_uuid(&conn, uuid) {
            Ok(u) => u,
            Err(db::users::GetUserError::NoSuchUser) => {
                return Err(Error::client_error(format!("No such user '{}'", uuid)));
            },
            Err(e) => {
                error!("error using uuid to get user: {:?}", e);
                return Err(Error::server_error(format!("Error getting user '{}'", uuid)));
            }
        };

        User::new(dbuser, &conn)
    }

    pub fn from_oauth_provider(conn: &PgConnection, provider: &oauth::Provider, oauth_id: &i32) -> Result<Self, Error> {
        let dbuser = match DBUser::from_oauth_provider(&conn, &provider, &oauth_id) {
            Ok(u) => u,
            Err(db::users::GetUserError::NoSuchUser) => {
                return Err(Error::client_error(format!("No such user '{}' for '{:?}'", oauth_id, provider)));
            },
            Err(e) => {
                error!("error using oauth provider to get user: {:?}", e);
                return Err(Error::server_error(format!("Erro getting user '{}' for '{:?}'", oauth_id, provider)));
            }
        };

        User::new(dbuser, &conn)
    }

    pub fn add_group(&self, conn: &PgConnection, group: Uuid) -> Result<(), Error> {
        let dbres = match self._dbuser {
            Some(ref u) => u.add_group(&conn, group),
            None => {
                return Err(Error::server_error("cannot add group: user not constructed with db reference".to_string()));
            }
        };

        dbres.map_err(|e| {
            Error::server_error(format!("error adding user to group: {:?}", e))
        })
    }
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AuthMetadata {
    pub github: Option<Result<GithubAuthMetadata, String>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GithubAuthMetadata {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GroupResp {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub public: bool,
}

impl GroupResp {
    pub fn list_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        db::groups::Group::list_all(conn)
            .map(|gs| {
                gs.into_iter().map(|g| (&g).into()).collect()
            })
            .map_err(|e| {
                Error::server_error(format!("error listing groups: {:?}", e))
            })
    }
}

impl<'a> From<&'a db::groups::Group> for GroupResp {
    fn from(g: &'a db::groups::Group) -> Self {
        GroupResp {
            uuid: g.uuid.simple().to_string(),
            name: g.name.clone(),
            description: g.description.clone(),
            public: g.public,
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AuthUserResp {
    UserResp(User),
    PartialUser(PartialUser),
}

pub struct CookieUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for CookieUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let uuid_ = {
            // ensure cookies is dropped before the PartialUser::from_request so we don't error out
            // on too many cookies
            let mut cookies = request.cookies();
            match cookies.get_private("user_uuid") {
                Some(uuid) => {
                    debug!("got user_uuid from cookie: {}", uuid);
                    match Uuid::parse_str(&uuid.value()) {
                        Ok(u) => Some(u),
                        Err(e) => {
                            error!("could not decode user's uuid: {}", e);
                            return Outcome::Failure((Status::InternalServerError, ()));
                        }
                    }
                }
                None => {
                    debug!("cookie had no user_uuid");
                    None
                }
            }
        };

        let db = request.guard::<rocket::State<db::Pool>>()?;
        let db = match db.get() {
            Ok(db) => db,
            Err(e) => {
                error!("error getting db: {}", e);
                return Outcome::Failure((Status::InternalServerError, ()));
            }
        };

        let uuid_ = match uuid_ {
            Some(u) => u,
            None => {
                return Outcome::Forward(());
            }
        };
        let dbuser = match DBUser::from_uuid(&*db, uuid_) {
            Err(db::users::GetUserError::NoSuchUser) => {
                return Outcome::Failure((Status::NotFound, ()));
            },
            Err(e) => {
                error!("error using uuid to get user: {:?}", e);
                return Outcome::Failure((Status::InternalServerError, ()));
            }
            Ok(u) => u,
        };

        match User::new(dbuser, &db) {
            Ok(u) => Outcome::Success(CookieUser(u)),
            Err(_) => Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}

pub struct OauthUser {
    pub user: User,
    pub scopes: Vec<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for OauthUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let raw_auth = match request.headers().get_one("authorization") {
            Some(s) => s,
            None => {
                debug!("No authorization header");
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };
        
        let auth: BearerAuth = match raw_auth.parse() {
            Ok(a) => a,
            Err(e) => {
                warn!("could not parse Authorization header: {:?}", e);
                return Outcome::Failure((Status::Unauthorized, ()));
            },
        };

        // split off the 'Bearer' prefix
        let token = auth.token.split_whitespace().last().unwrap_or("");
        if token == "" {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let hydra = request.guard::<rocket::State<hydra::client::ClientBuilder>>()?;
        let handle = request.guard::<Handle>()?;
        let client = hydra.build(&handle.into()).client();

        let res = client.o_auth2_api().introspect_o_auth2_token(&token, "")
            .then(|res| {
                match res {
                    Ok(user) => {
                        if !user.active().unwrap_or(&false) {
                            return Err(Outcome::Failure((Status::Unauthorized, ())));
                        }
                        Ok((user.sub().unwrap().clone(), user.scope().unwrap().clone()))
                    }
                    Err(e) => {
                        error!("oauth2 introspect error: {:?}", e);
                        Err(Outcome::Failure((Status::InternalServerError, ())))
                    }
                }
            }).and_then(|(uuid, scopes)| {
                let db = match request.guard::<rocket::State<db::Pool>>() {
                    rocket::Outcome::Success(p) => p,
                    rocket::Outcome::Failure(f) => {
                        return Err(rocket::Outcome::Failure(f));
                    },
                    rocket::Outcome::Forward(f) => {
                        return Err(rocket::Outcome::Forward(f));
                    },
                };
                let db = match db.get() {
                    Ok(db) => db,
                    Err(e) => {
                        error!("error getting db: {}", e);
                        return Err(Outcome::Failure((Status::InternalServerError, ())));
                    }
                };

                let uuid: Uuid = match uuid.parse() {
                    Err(e) => {
                        warn!("uuid parse error: {:?}", e);
                        return Err(Outcome::Failure((Status::BadRequest, ())));
                    }
                    Ok(u) => u,
                };

                match User::from_uuid(uuid, &db) {
                    Err(e) => {
                        Err(Outcome::Failure((e.status(), ())))
                    }
                    Ok(u) => Ok(Outcome::Success(OauthUser{
                        user: u,
                        scopes: scopes.split_whitespace().map(String::from).collect(),
                    }))
                }
            });

        match multi_reactor_drifting::run(res) {
            Ok(o) => o,
            Err(e) => {
                e
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub partial_user: PartialUser,
    pub username: String,
    pub email: String,
}

impl CreateUserRequest {
    pub fn create(&self, conn: &PgConnection) -> Result<User, Error> {
        if self.username.len() == 0 {
            return Err(Error::client_error("Name cannot be blank".to_string()));
        }
        if self.email.len() == 0 {
            return Err(Error::client_error(
                "Email cannot be blank".to_string(),
            ));
        }

        let mut auth_meta = AuthMetadata{
            github: None,
        };

        let create_res = match self.partial_user.provider {
            oauth::Provider::Github => {
                let gh = match get_github_user(&self.partial_user.access_token) {
                    Ok(u) => u,
                    Err(e) => return Err(e).into(),
                };
                auth_meta.github = Some(Ok(GithubAuthMetadata{
                    username: gh.login.clone(),
                }));
                db::users::NewUser{
                    username: &self.username,
                    email: &self.email,
                }.insert_github(&*conn, db::users::NewGithubAccount{
                    id: self.partial_user.provider_id,
                    username: &gh.login,
                })
            }
            oauth::Provider::Local => db::users::NewUser{
                username: &self.username,
                email: &self.email,
            }.insert_local(&*conn, db::users::NewLocalAccount{
                id: self.partial_user.provider_id,
            })
        };

        match create_res {
            Err(e) => {
                match e {
                    DieselErr::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, e) => {
                        match e.constraint_name() {
                            Some("users_username_key") => {
                                Err(Error::client_error(format!("Could not create account; username '{}' already exists.", self.username)))
                            }
                            Some("github_accounts_pkey") => {
                                Err(Error::client_error(format!("Could not create account; Github account with id {} already associated with a user.", self.partial_user.provider_id)))
                            }
                            _ => {
                                error!("uniqueness violation case missed: {:?}: {:?}, {:?}", e, e.table_name(), e.column_name());
                                Err(Error::client_error("An unknown uniqueness violation happened, sorry :(".to_string()))
                            }
                        }
                    },
                    _ => {
                        error!("error creating user account: {}", e);
                        Err(Error::server_error("Could not create account: please contact the administrator if this persists".to_string()))
                    }
                }
            }
            Ok(newuser) => {
                User::new(newuser, conn)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user_uuid: String,

    pub username:  Option<String>,
    pub email:     Option<String>,
    pub add_groups: Option<Vec<Uuid>>,
    pub remove_groups: Option<Vec<Uuid>>,
}

impl UpdateUserRequest {
    // changes_only_user_controlled_fields should return whether this update request only makes
    // changes users are allowed to make, such as changing their username and email.
    // For now, users are not allowed to change their own groups, only admins can do that.
    // This function is used for policy checking.
    pub fn changes_only_user_controlled_fields(&self) -> bool {
        return self.add_groups.is_none() && self.remove_groups.is_none();
    }

    pub fn update(&self, conn: &PgConnection) -> Result<User, Error> {
        use diesel::prelude::*;
        use db::schema::users;

        let user_uuid = Uuid::parse_str(&self.user_uuid)
            .map_err(|e| Error::client_error(format!("invalid uuid: {}", e)))?;
        let user_target = users::table.filter(users::uuid.eq(user_uuid));

        let dbuser: DBUser = diesel::update(user_target).set(&db::users::UpdateUser{
            username: self.username.clone(),
            email: self.email.clone(),
        }).get_result(conn)
        .map_err(|e| Error::server_error(format!("error updating user in db: {}", e)))?;

        // and now update groups if needed
        if let Some(ref new_groups) = self.add_groups {
            dbuser.add_groups(&conn, new_groups)
                .map_err(|e| Error::server_error(format!("error adding groups: {}", e)))?;
        }

        if let Some(ref rm_groups) = self.remove_groups {
            dbuser.delete_groups(&conn, rm_groups)
                .map_err(|e| Error::server_error(format!("error removing groups: {}", e)))?;
        }

        User::new(dbuser, &conn)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub public: bool,
    pub description: String,
}

impl CreateGroupRequest {
    pub fn create(&self, conn: &PgConnection) -> Result<GroupResp, Error> {
        let g = NewGroup{
            name: self.name.clone(),
            public: self.public,
            description: self.description.clone(),
            uuid: None,
        };

        use diesel::prelude::*;
        use db::schema::groups::dsl::*;

        let db_groups: Vec<db::groups::Group> = diesel::insert_into(groups)
            .values(&g)
            .on_conflict_do_nothing()
            .get_results(conn)
            .map_err(|e| {
                Error::server_error(format!("error creating group {}: {}", g.name, e))
            })?;

        match db_groups.first() {
            None => Err(Error::server_error("empty groups returned, but no db error".to_string())),
            Some(g) => Ok(g.into()),
        }
    }
}
