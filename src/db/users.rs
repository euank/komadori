use db;
use db::groups::Group;
use diesel::prelude::*;
use diesel;
use std::time::{SystemTime, Instant};
use oauth;
use uuid::Uuid;
use db::schema::{users, groups, users_groups, github_accounts, local_accounts};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(Insertable)]
#[table_name = "github_accounts"]
pub struct NewGithubAccount<'a> {
    pub id: i32,
    pub username: &'a str,
}

#[derive(Insertable)]
#[table_name = "local_accounts"]
pub struct NewLocalAccount {
    pub id: i32,
}


#[derive(Debug, Clone, Queryable, Identifiable, PartialEq)]
pub struct User {
    id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub role: Option<String>,
    pub email: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[table_name="users"]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct GithubAccount {
    pub id: i32,
    user_id: i32,
    access_token: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug)]
pub enum GetUserError {
    DbError(diesel::result::Error),
    NoSuchUser,
}

impl<'a> NewUser<'a> {
    pub fn insert_github(self, conn: &diesel::PgConnection, gh: NewGithubAccount) -> Result<User, diesel::result::Error> {
        // TODO: error handling, e.g. detect client vs server errors (such as uniqueness constraints
        // being client, and db conn errs being server)
        let create_res = (&*conn).transaction::<_, diesel::result::Error, _>(|| {
            use diesel;
            use diesel::prelude::*;
            use db::schema::users::dsl::*;
            use db::schema::github_accounts;
            let newuser: User = diesel::insert_into(users).values(&self)
                .get_result(&*conn)?;

            diesel::insert_into(github_accounts::table)
                .values((&gh, github_accounts::user_id.eq(newuser.id)))
                .execute(&*conn)?;

            Ok(newuser)
        });
        create_res
    }

    pub fn insert_local(self, conn: &diesel::PgConnection, local: NewLocalAccount) -> Result<User, diesel::result::Error> {
        // TODO: error handling, e.g. detect client vs server errors (such as uniqueness constraints
        // being client, and db conn errs being server)
        let create_res = (&*conn).transaction::<_, diesel::result::Error, _>(|| {
            use diesel;
            use diesel::prelude::*;
            use db::schema::users::dsl::*;
            use db::schema::local_accounts;
            let newuser: User = diesel::insert_into(users).values(&self)
                .get_result(&*conn)?;

            diesel::insert_into(local_accounts::table)
                .values((&local, local_accounts::user_id.eq(newuser.id)))
                .execute(&*conn)?;

            Ok(newuser)
        });
        create_res
    }
}


impl User {
    pub fn list(conn: &diesel::PgConnection) -> Result<Vec<Self>, GetUserError> {
        use diesel::prelude::*;
        use db::schema::users::dsl::*;

        match users.load::<User>(conn) {
            Ok(us) => Ok(us),
            Err(e) => {
                error!("error getting all users {}", e);
                Err(GetUserError::DbError(e))
            }
        }
    }

    pub fn from_uuid(conn: &diesel::PgConnection, uuid_: Uuid) -> Result<Self, GetUserError> {
        use diesel::prelude::*;
        use db::schema::users::dsl::*;
        match users.filter(uuid.eq(uuid_)).limit(1).load::<User>(conn) {
            Ok(u) => match u.first() {
                Some(u) => Ok(u.clone()),
                None => {
                    error!("error getting user {}", uuid_);
                    Err(GetUserError::NoSuchUser)
                }
            },
            Err(e) => {
                error!("error getting user {}: {}", uuid_, e);
                Err(GetUserError::DbError(e))
            }
        }
    }

    pub fn from_oauth_provider(
        conn: &diesel::PgConnection,
        provider: &oauth::Provider,
        provider_id: &i32,
    ) -> Result<Self, GetUserError> {
        // Compile-check that we can assume github's the only provider
        use db::schema::github_accounts;
        use db::schema::local_accounts;

        use diesel::prelude::*;
        use db::schema::users::dsl::*;
        match {
            let timer = Instant::now();
            let res = match provider {
                oauth::Provider::Github => {
                    users
                        .inner_join(github_accounts::table)
                        .select(users::all_columns())
                        .filter(github_accounts::id.eq(provider_id))
                        .load::<User>(conn)
                }
                oauth::Provider::Local => {
                    users
                        .inner_join(local_accounts::table)
                        .select(users::all_columns())
                        .filter(local_accounts::id.eq(provider_id))
                        .load::<User>(conn)
                }
            };
            debug!(
                "Partial user to user query took {}",
                (timer.elapsed().as_secs() as f64 + timer.elapsed().subsec_nanos() as f64 * 1e-9)
            );
            res
        } {
            Ok(u) => match u.first() {
                Some(u) => Ok(u.clone()),
                None => Err(GetUserError::NoSuchUser),
            },
            Err(e) => Err(GetUserError::DbError(e)),
        }
    }

    pub fn groups(&self, conn: &db::PgConnection) -> Result<Vec<Group>, String> {
        use diesel::prelude::*;

        groups::table.left_join(users_groups::table.on(users_groups::user_id.eq(self.id)))
            .select(groups::table::all_columns())
            .filter(users_groups::user_id.eq(self.id))
            .load::<Group>(&*conn)
            .map_err(|e| {
                format!("error loading groups: {}", e)
            })
    }

    pub fn add_group(&self, conn: &db::PgConnection, group: Uuid) -> Result<(), diesel::result::Error> {
        let group_id = groups::table
            .select(groups::id)
            .filter(groups::uuid.eq(group))
            .first::<i32>(conn)?;

        diesel::insert_into(users_groups::table)
            .values((
                users_groups::user_id.eq(self.id),
                users_groups::group_id.eq(group_id),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn add_groups(&self, conn: &db::PgConnection, groups: &Vec<Uuid>) -> Result<(), diesel::result::Error> {
        if groups.len() == 0 {
            return Ok(());
        }
        use diesel::sql_types::Bool;
        let mut eqs = groups.into_iter().map(|g| groups::uuid.eq(g));
        let head: Box<BoxableExpression<_, _, SqlType = Bool>> = Box::new(eqs.next().unwrap()); // safe because we asserted len > 0 above
        let or_expr = eqs
            .fold(head, |sum: Box<BoxableExpression<_, _, SqlType = Bool>>, expr| {
                Box::new(sum.or(expr))
            });

        let group_ids = groups::table
            .select((groups::id, diesel::dsl::sql(&format!("{} as user_id", self.id))))
            .filter(or_expr);

        diesel::insert_into(users_groups::table)
            .values(group_ids)
            .into_columns((
                users_groups::group_id,
                users_groups::user_id,
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn delete_groups(&self, conn: &db::PgConnection, groups: &Vec<Uuid>) -> Result<(), diesel::result::Error> {
        if groups.len() == 0 {
            return Ok(());
        }
        use diesel::sql_types::Bool;
        let mut eqs = groups.into_iter().map(|g| groups::uuid.eq(g));
        let head: Box<BoxableExpression<_, _, SqlType = Bool>> = Box::new(eqs.next().unwrap()); // safe because we asserted len > 0 above
        let or_expr = eqs
            .fold(head, |sum: Box<BoxableExpression<_, _, SqlType = Bool>>, expr| {
                Box::new(sum.or(expr))
            });

        let group_ids = groups::table
            .select(groups::id)
            .filter(or_expr);

        diesel::delete(users_groups::table.filter(
                users_groups::user_id.eq(self.id).and(
                    users_groups::group_id.eq_any(group_ids)
                )
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn github_account(&self, conn: &db::PgConnection) -> Result<GithubAccount, String> {
        use diesel::prelude::*;

        github_accounts::table.left_join(users::table.on(github_accounts::user_id.eq(self.id)))
            .select(github_accounts::table::all_columns())
            .first::<GithubAccount>(&*conn)
            .map_err(|e| {
                format!("error loading groups: {}", e)
            })
    }
}
