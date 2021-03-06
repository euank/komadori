/* 
 * Hydra OAuth2 & OpenID Connect Server
 *
 * Please refer to the user guide for in-depth documentation: https://ory.gitbooks.io/hydra/content/   Hydra offers OAuth 2.0 and OpenID Connect Core 1.0 capabilities as a service. Hydra is different, because it works with any existing authentication infrastructure, not just LDAP or SAML. By implementing a consent app (works with any programming language) you build a bridge between Hydra and your authentication infrastructure. Hydra is able to securely manage JSON Web Keys, and has a sophisticated policy-based access control you can use if you want to. Hydra is suitable for green- (new) and brownfield (existing) projects. If you are not familiar with OAuth 2.0 and are working on a greenfield project, we recommend evaluating if OAuth 2.0 really serves your purpose. Knowledge of OAuth 2.0 is imperative in understanding what Hydra does and how it works.   The official repository is located at https://github.com/ory/hydra   ### Important REST API Documentation Notes  The swagger generator used to create this documentation does currently not support example responses. To see request and response payloads click on **\"Show JSON schema\"**: ![Enable JSON Schema on Apiary](https://storage.googleapis.com/ory.am/hydra/json-schema.png)   The API documentation always refers to the latest tagged version of ORY Hydra. For previous API documentations, please refer to https://github.com/ory/hydra/blob/<tag-id>/docs/api.swagger.yaml - for example:  0.9.13: https://github.com/ory/hydra/blob/v0.9.13/docs/api.swagger.yaml 0.8.1: https://github.com/ory/hydra/blob/v0.8.1/docs/api.swagger.yaml
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineResponse401 {
  #[serde(rename = "code")]
  code: Option<i64>,
  #[serde(rename = "details")]
  details: Option<Vec<::std::collections::HashMap<String, Value>>>,
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "reason")]
  reason: Option<String>,
  #[serde(rename = "request")]
  request: Option<String>,
  #[serde(rename = "status")]
  status: Option<String>
}

impl InlineResponse401 {
  pub fn new() -> InlineResponse401 {
    InlineResponse401 {
      code: None,
      details: None,
      message: None,
      reason: None,
      request: None,
      status: None
    }
  }

  pub fn set_code(&mut self, code: i64) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: i64) -> InlineResponse401 {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&i64> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_details(&mut self, details: Vec<::std::collections::HashMap<String, Value>>) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: Vec<::std::collections::HashMap<String, Value>>) -> InlineResponse401 {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&Vec<::std::collections::HashMap<String, Value>>> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> InlineResponse401 {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> InlineResponse401 {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_request(&mut self, request: String) {
    self.request = Some(request);
  }

  pub fn with_request(mut self, request: String) -> InlineResponse401 {
    self.request = Some(request);
    self
  }

  pub fn request(&self) -> Option<&String> {
    self.request.as_ref()
  }

  pub fn reset_request(&mut self) {
    self.request = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> InlineResponse401 {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



