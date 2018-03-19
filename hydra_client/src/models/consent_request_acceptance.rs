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
pub struct ConsentRequestAcceptance {
  /// AccessTokenExtra represents arbitrary data that will be added to the access token and that will be returned on introspection and warden requests.
  #[serde(rename = "accessTokenExtra")]
  access_token_extra: Option<::std::collections::HashMap<String, Value>>,
  /// A list of scopes that the user agreed to grant. It should be a subset of requestedScopes from the consent request.
  #[serde(rename = "grantScopes")]
  grant_scopes: Option<Vec<String>>,
  /// IDTokenExtra represents arbitrary data that will be added to the ID token. The ID token will only be issued if the user agrees to it and if the client requested an ID token.
  #[serde(rename = "idTokenExtra")]
  id_token_extra: Option<::std::collections::HashMap<String, Value>>,
  /// Subject represents a unique identifier of the user (or service, or legal entity, ...) that accepted the OAuth2 request.
  #[serde(rename = "subject")]
  subject: Option<String>
}

impl ConsentRequestAcceptance {
  pub fn new() -> ConsentRequestAcceptance {
    ConsentRequestAcceptance {
      access_token_extra: None,
      grant_scopes: None,
      id_token_extra: None,
      subject: None
    }
  }

  pub fn set_access_token_extra(&mut self, access_token_extra: ::std::collections::HashMap<String, Value>) {
    self.access_token_extra = Some(access_token_extra);
  }

  pub fn with_access_token_extra(mut self, access_token_extra: ::std::collections::HashMap<String, Value>) -> ConsentRequestAcceptance {
    self.access_token_extra = Some(access_token_extra);
    self
  }

  pub fn access_token_extra(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.access_token_extra.as_ref()
  }

  pub fn reset_access_token_extra(&mut self) {
    self.access_token_extra = None;
  }

  pub fn set_grant_scopes(&mut self, grant_scopes: Vec<String>) {
    self.grant_scopes = Some(grant_scopes);
  }

  pub fn with_grant_scopes(mut self, grant_scopes: Vec<String>) -> ConsentRequestAcceptance {
    self.grant_scopes = Some(grant_scopes);
    self
  }

  pub fn grant_scopes(&self) -> Option<&Vec<String>> {
    self.grant_scopes.as_ref()
  }

  pub fn reset_grant_scopes(&mut self) {
    self.grant_scopes = None;
  }

  pub fn set_id_token_extra(&mut self, id_token_extra: ::std::collections::HashMap<String, Value>) {
    self.id_token_extra = Some(id_token_extra);
  }

  pub fn with_id_token_extra(mut self, id_token_extra: ::std::collections::HashMap<String, Value>) -> ConsentRequestAcceptance {
    self.id_token_extra = Some(id_token_extra);
    self
  }

  pub fn id_token_extra(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.id_token_extra.as_ref()
  }

  pub fn reset_id_token_extra(&mut self) {
    self.id_token_extra = None;
  }

  pub fn set_subject(&mut self, subject: String) {
    self.subject = Some(subject);
  }

  pub fn with_subject(mut self, subject: String) -> ConsentRequestAcceptance {
    self.subject = Some(subject);
    self
  }

  pub fn subject(&self) -> Option<&String> {
    self.subject.as_ref()
  }

  pub fn reset_subject(&mut self) {
    self.subject = None;
  }

}



