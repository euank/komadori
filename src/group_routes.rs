use db;
use types::{CookieUser, CreateGroupRequest, GroupResp};
use errors::{JsonResult, Error};
use rocket_contrib::json::Json;
use rocket;
use policy;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_group]
}

#[post("/group/create", format = "application/json", data = "<req>")]
fn create_group(
    conn: db::Conn,
    req: Json<CreateGroupRequest>,
    user: CookieUser,
) -> JsonResult<GroupResp> {
    // TODO: this should not require admin, but rather there should be a policy check of some kind.
    // Punting for now.
    unimplemented!()
}
