extern crate rocket;
extern crate komadori;

use std::env;
use komadori::*;
use rocket::local::*;
use rocket::http::Status;

fn rocket() -> rocket::Rocket {
    let hydra_conf = {
        let url = must_env("HYDRA_URL");
        let client_id = must_env("HYDRA_CLIENT_ID");
        let client_secret = must_env("HYDRA_CLIENT_SECRET");
        komadori::HydraConfig {
            url: url,
            client_id: client_id,
            client_secret: client_secret,
        }
    };

    let pool = {
        let uri = &must_env("DATABASE_URL");
        komadori::db::db_pool(uri).expect("error connecting to database")
    };

    komadori::rocket(komadori::Config{
        base_url: "".to_string(),
        hydra: hydra_conf,
        github_provider: None,
        local_provider: Some(provider::local::Local::new()),
        pool: pool,
        environment: Environment::Dev,
    })
}

#[test]
fn it_lets_users_login() {
    let rkt = rocket();
    let c = Client::new(rkt).unwrap();

    let not_logged_in = c.get("/user").dispatch();
    assert_eq!(not_logged_in.status(), Status::NotFound);

    // Now use the dev flow to create an account
    let mut req = c.post("/user/create");
    req.set_body(r#"{
        "username": "test_account",
        "email": "test@email.asdf",
        "partial_user": {
            "provider": "local",
            "provider_id": 1,
            "provider_name": "testuser",
            "access_token": "foo"
        }
    }"#);
    req.add_header(rocket::http::ContentType::JSON);

    let mut resp = req.dispatch();
    println!("{:?}", resp.body_string());
    println!("{:?}", resp.status());
}


fn must_env(var: &str) -> String {
    env::var(var).expect(&format!("Environment variable '{}' must be set", var))
}
