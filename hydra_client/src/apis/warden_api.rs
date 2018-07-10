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

pub struct WardenApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> WardenApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> WardenApiClient<C> {
        WardenApiClient {
            configuration: configuration,
        }
    }
}

pub trait WardenApi {
    fn add_members_to_group(&self, id: &str, group_members: ::models::GroupMembers) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_group(&self, group: ::models::Group) -> Box<Future<Item = ::models::Group, Error = Error<serde_json::Value>>>;
    fn delete_group(&self, id: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn does_warden_allow_access_request(&self, warden_access_request: ::models::WardenAccessRequest) -> Box<Future<Item = ::models::WardenAccessRequestResponse, Error = Error<serde_json::Value>>>;
    fn does_warden_allow_token_access_request(&self, warden_token_access_request: ::models::WardenTokenAccessRequest) -> Box<Future<Item = ::models::WardenTokenAccessRequestResponse, Error = Error<serde_json::Value>>>;
    fn get_group(&self, id: &str) -> Box<Future<Item = ::models::Group, Error = Error<serde_json::Value>>>;
    fn list_groups(&self, member: &str, limit: i64, offset: i64) -> Box<Future<Item = Vec<::models::Group>, Error = Error<serde_json::Value>>>;
    fn remove_members_from_group(&self, id: &str, group_members: ::models::GroupMembers) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>WardenApi for WardenApiClient<C> {
    fn add_members_to_group(&self, id: &str, group_members: ::models::GroupMembers) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/warden/groups/{id}/members".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("id".to_string(), id.to_string())
            .with_body_param(group_members)
            .returns_nothing(true)
            .execute(self.configuration.borrow())
    }

    fn create_group(&self, group: ::models::Group) -> Box<Future<Item = ::models::Group, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/warden/groups".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_body_param(group)
            .execute(self.configuration.borrow())
    }

    fn delete_group(&self, id: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/warden/groups/{id}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("id".to_string(), id.to_string())
            .returns_nothing(true)
            .execute(self.configuration.borrow())
    }

    fn does_warden_allow_access_request(&self, warden_access_request: ::models::WardenAccessRequest) -> Box<Future<Item = ::models::WardenAccessRequestResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/warden/allowed".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_body_param(warden_access_request)
            .execute(self.configuration.borrow())
    }

    fn does_warden_allow_token_access_request(&self, warden_token_access_request: ::models::WardenTokenAccessRequest) -> Box<Future<Item = ::models::WardenTokenAccessRequestResponse, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/warden/token/allowed".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_body_param(warden_token_access_request)
            .execute(self.configuration.borrow())
    }

    fn get_group(&self, id: &str) -> Box<Future<Item = ::models::Group, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/warden/groups/{id}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("id".to_string(), id.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_groups(&self, member: &str, limit: i64, offset: i64) -> Box<Future<Item = Vec<::models::Group>, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/warden/groups".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_query_param("member".to_string(), member.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("offset".to_string(), offset.to_string())
            .execute(self.configuration.borrow())
    }

    fn remove_members_from_group(&self, id: &str, group_members: ::models::GroupMembers) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/warden/groups/{id}/members".to_string())
            .with_auth(__internal_request::Auth::Oauth)
            .with_path_param("id".to_string(), id.to_string())
            .with_body_param(group_members)
            .returns_nothing(true)
            .execute(self.configuration.borrow())
    }

}
