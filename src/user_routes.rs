use db;
use uuid::Uuid;
use types::{User, CookieUser, CreateUserRequest, UpdateUserRequest, AuthUserResp};
use rocket::State;
use rocket;
use policy;
use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use errors::Error;
use errors::JsonResult;
use provider::{ProviderSet, ProviderAuthRequest};

pub fn routes() -> Vec<rocket::Route> {
    routes![auth_user, create_user, logout_user, get_user, get_user_by_uuid, update_user]
}

#[get("/user", format = "application/json")]
pub fn get_user(user: CookieUser) -> JsonResult<User> {
    Ok(user.0).into()
}

#[get("/user/<uuid>", format = "application/json")]
pub fn get_user_by_uuid(conn: db::Conn, user: CookieUser, uuid: String) -> JsonResult<User> {
    let uuid = match Uuid::parse_str(&uuid) {
        Ok(u) => u,
        Err(e) => {
            return Err(Error::client_error(format!("invalid uuid: {}", e))).into();
        }
    };
    if !policy::is_allowed(user.0, policy::Action::GetUser(&uuid)) {
        return Err(Error::client_error("permission denied".to_string())).into();
    }
    User::from_uuid(uuid, &conn).into()
}


#[post("/user/create", format = "application/json", data = "<req>")]
pub fn create_user(
    conn: db::Conn,
    req: Json<CreateUserRequest>,
    mut cookies: Cookies,
) -> JsonResult<User> {
    let res = match req.create(&*conn) {
        Ok(u) => {
            cookies.add_private(Cookie::new(
                "user_uuid".to_owned(),
                u.uuid.clone(),
            ));
            cookies.remove_private(Cookie::named("oauth_token"));
            Ok(u)
        }
        x => x
    };
    res.into()
}

#[post("/user/update", format = "application/json", data = "<req>")]
pub fn update_user(
    conn: db::Conn,
    user: CookieUser,
    req: Json<UpdateUserRequest>,
) -> JsonResult<User> {
    if !policy::is_allowed(user.0, policy::Action::UpdateUser(&req)) {
        return Err(Error::client_error("permission denied".to_string())).into();
    }

    req.update(&*conn).into()
}

#[derive(Serialize)]
pub struct UserLogoutResponse {}

#[get("/user/logout")]
pub fn logout_user(mut cookies: Cookies) -> JsonResult<UserLogoutResponse> {
    cookies.remove_private(Cookie::named("oauth_token"));
    cookies.remove_private(Cookie::named("user_uuid"));
    Ok(UserLogoutResponse {}).into()
}

#[post("/user/auth", format = "application/json", data = "<req>")]
pub fn auth_user(mut cookies: Cookies, conn: db::Conn, providers: State<ProviderSet>, req: Json<ProviderAuthRequest>) -> JsonResult<AuthUserResp> {
    let pu = match providers.partial_user(&*req) {
        Ok(pu) => pu,
        Err(e) => { return Err(e).into() },
    };

    let user = match User::from_oauth_provider(&conn, &pu.provider, &pu.provider_id) {
        Ok(u) => {
            cookies.add_private(Cookie::new(
                "user_uuid".to_owned(),
                u.uuid.clone(),
            ));
            u
        }
        Err(e) => {
            debug!("error getting user from partial user; assuming user doesn't exist: {:?}", e);
            // TODO: better error handling for client vs server errs
            return Ok(AuthUserResp::PartialUser(pu)).into();
        }
    };
    Ok(AuthUserResp::UserResp(user)).into()
}
