use hyper;
use hydra_oauthed_client::HydraClientWrapper;
use hydra_client;
use hydra_client::apis::WardenApi;
use futures;
use futures::future::Future;

pub const USER_GROUP: &str = "users";
pub const DEV_GROUP: &str = "wobdevs";
pub const ADMIN_GROUP: &str = "admins";

fn all_groups() -> Vec<&'static str> {
    vec![USER_GROUP, DEV_GROUP]
}

pub fn initialize_groups(hydra: HydraClientWrapper<hyper::client::HttpConnector>) -> Box<Future<Error = String, Item = Vec<()>>> {
    // This might be racy, but fortunately we only run one of these at a time anyways
    let group_res = all_groups().into_iter().map(|group| {
        hydra.client().warden_api().get_group(group).then(|res| {
            match res {
                Err(e) => {
                    debug!(
                        "warden: got error {:?}, assuming group {} doesn't exist",
                        e, group
                    );
                    let f: Box<futures::Future<Error = String, Item = ()>> = Box::new(hydra.client().warden_api().create_group(hydra_client::models::Group::new().with_id(group.to_string())).map(|g| {
                        info!("created group: {}", g.id().unwrap());
                        ()
                    }).map_err(|e| format!("error creating group {}: {:?}", group, e)));
                    f
                },
                Ok(_) => {
                    let f: Box<futures::Future<Error = String, Item = ()>> = Box::new(futures::future::ok(()));
                    f
                }
            }
        })
        .map_err(|e| {
            format!("error creating group: {:?}", e)
        })
    }).collect::<Vec<_>>();

    Box::new(futures::future::join_all(group_res))
}
