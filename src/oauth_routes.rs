use std::collections::HashMap;
use serde_json;
use rocket;
use rocket::State;
use rocket_contrib::json::Json;
use errors::{Error, JsonResult};
use hydra;
use hydra_client;
use types::CookieUser;
use multi_reactor_drifting::{Handle, Future};
use futures::Future as _______________;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_consent_info, accept_consent, reject_consent]
}

#[derive(FromForm)]
pub struct ConsentId {
    id: String,
}

#[derive(Serialize)]
pub struct ConsentInfo {
    id: String,
    client: String,
    scopes: Vec<String>,
    redirect: String,
}

#[get("/oauth/consent?<consent_id>")]
pub fn get_consent_info(
    consent_id: ConsentId,
    hydra: State<hydra::client::ClientBuilder>,
    handle: Handle,
) -> Future<JsonResult<ConsentInfo>, ()> {
    let client = hydra.build(&handle.into()).client();
    let json_future = client.o_auth2_api().get_o_auth2_consent_request(&consent_id.id)
        .then(|res| {
            // Outer 'Ok' is the Future<> one, inner Ok/Err is the JsonResult one
            Ok(match res {
                Ok(info) => Ok(ConsentInfo {
                    client: info.client_id().unwrap().to_string(),
                    scopes: info.requested_scopes().unwrap().clone(),
                    redirect: info.redirect_url().unwrap().to_string(),
                    id: info.id().unwrap().to_string(),
                }),
                Err(e) => Err(Error::client_error(format!(
                    "error getting info about consent request: {:?}",
                    e
                ))),
            }.into())
        });

    Future(Box::new(json_future))
}

#[derive(Deserialize)]
pub struct AcceptConsent {
    id: String,
    scopes: Vec<String>,
}

#[post("/oauth/consent/accept", format = "application/json", data = "<req>")]
pub fn accept_consent(
    req: Json<AcceptConsent>,
    user: CookieUser,
    hydra: State<hydra::client::ClientBuilder>,
    handle: Handle,
) -> Future<JsonResult<()>, ()> {
    let user = user.0;
    let client = hydra.build(&handle.into()).client();
    // note: email is a required 'extra' attribute for some oauth proxies; set it to the uuid to
    // allow this oidc to better interoperate with those even though technically this isn't right.
    let mut extra_claims = HashMap::new();
    extra_claims.insert("email".to_owned(), serde_json::Value::String(user.uuid.clone()));

    let accept = hydra_client::models::ConsentRequestAcceptance::new()
        .with_id_token_extra(extra_claims)
        .with_subject(user.uuid)
        .with_grant_scopes(req.scopes.clone());
    let resp = client.o_auth2_api()
        .accept_o_auth2_consent_request(&req.id, accept)
        .then(|res| {
            Ok(match res {
                Ok(()) => Ok(()),
                Err(e) => Err(Error::client_error(format!(
                    "error accepting consent: {:?}",
                    e
                ))),
            }.into())
        }).map_err(|e: Result<(), Error>| {
            unreachable!(".then should have squashed Err: {:?}", e)
        });

    Future(Box::new(resp))
}

#[derive(Deserialize)]
pub struct RejectConsent {
    id: String,
    reason: Option<String>,
}

#[post("/oauth/consent/reject", format = "application/json", data = "<req>")]
pub fn reject_consent(
    req: Json<RejectConsent>,
    _user: CookieUser,
    hydra: State<hydra::client::ClientBuilder>,
    handle: Handle,
) -> Future<JsonResult<()>, ()> {
    let client = hydra.build(&handle.into()).client();
    let accept = hydra_client::models::ConsentRequestRejection::new()
        .with_reason(req.reason.clone().unwrap_or("user rejected consent".to_string()));
    let reject = client.o_auth2_api().reject_o_auth2_consent_request(&req.id, accept).then(|res| {
        Ok(match res {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::client_error(format!(
                "error accepting consent: {:?}",
                e
            ))),
        }.into())
    }).map_err(|_: ()| unreachable!());

    Future(Box::new(reject))
}
