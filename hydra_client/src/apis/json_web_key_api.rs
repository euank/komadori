/* 
 * ORY Hydra - Cloud Native OAuth 2.0 and OpenID Connect Server
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here. Keep in mind that this document reflects the latest branch, always. Support for versioned documentation is coming in the future.
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct JsonWebKeyApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> JsonWebKeyApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> JsonWebKeyApiClient<C> {
        JsonWebKeyApiClient {
            configuration: configuration,
        }
    }
}

pub trait JsonWebKeyApi {
    fn create_json_web_key_set(&self, set: &str, json_web_key_set_generator_request: ::models::JsonWebKeySetGeneratorRequest) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>>;
    fn delete_json_web_key(&self, kid: &str, set: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_json_web_key_set(&self, set: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_json_web_key(&self, kid: &str, set: &str) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>>;
    fn get_json_web_key_set(&self, set: &str) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>>;
    fn update_json_web_key(&self, kid: &str, set: &str, json_web_key: ::models::JsonWebKey) -> Box<Future<Item = ::models::JsonWebKey, Error = Error<serde_json::Value>>>;
    fn update_json_web_key_set(&self, set: &str, json_web_key_set: ::models::JsonWebKeySet) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>JsonWebKeyApi for JsonWebKeyApiClient<C> {
    fn create_json_web_key_set(&self, set: &str, json_web_key_set_generator_request: ::models::JsonWebKeySetGeneratorRequest) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/keys/{set}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("set".to_string(), set.to_string())
            .with_body_param(json_web_key_set_generator_request)
            .execute(self.configuration.borrow())
    }

    fn delete_json_web_key(&self, kid: &str, set: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/keys/{set}/{kid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("kid".to_string(), kid.to_string())
            .with_path_param("set".to_string(), set.to_string())
            .returns_nothing(true)
            .execute(self.configuration.borrow())
    }

    fn delete_json_web_key_set(&self, set: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/keys/{set}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("set".to_string(), set.to_string())
            .returns_nothing(true)
            .execute(self.configuration.borrow())
    }

    fn get_json_web_key(&self, kid: &str, set: &str) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/keys/{set}/{kid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("kid".to_string(), kid.to_string())
            .with_path_param("set".to_string(), set.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_json_web_key_set(&self, set: &str) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/keys/{set}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("set".to_string(), set.to_string())
            .execute(self.configuration.borrow())
    }

    fn update_json_web_key(&self, kid: &str, set: &str, json_web_key: ::models::JsonWebKey) -> Box<Future<Item = ::models::JsonWebKey, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/keys/{set}/{kid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("kid".to_string(), kid.to_string())
            .with_path_param("set".to_string(), set.to_string())
            .with_body_param(json_web_key)
            .execute(self.configuration.borrow())
    }

    fn update_json_web_key_set(&self, set: &str, json_web_key_set: ::models::JsonWebKeySet) -> Box<Future<Item = ::models::JsonWebKeySet, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/keys/{set}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("set".to_string(), set.to_string())
            .with_body_param(json_web_key_set)
            .execute(self.configuration.borrow())
    }

}
