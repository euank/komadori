/* 
 * ORY Hydra - Cloud Native OAuth 2.0 and OpenID Connect Server
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here. Keep in mind that this document reflects the latest branch, always. Support for versioned documentation is coming in the future.
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

/// WardenTokenAccessRequestResponse : The warden access request (with token) response

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WardenTokenAccessRequestResponse {
  /// Extra represents arbitrary session data.
  #[serde(rename = "accessTokenExtra")]
  access_token_extra: Option<::std::collections::HashMap<String, Value>>,
  /// Allowed is true if the request is allowed and false otherwise.
  #[serde(rename = "allowed")]
  allowed: Option<bool>,
  /// ClientID is the id of the OAuth2 client that requested the token.
  #[serde(rename = "clientId")]
  client_id: Option<String>,
  /// ExpiresAt is the expiry timestamp.
  #[serde(rename = "expiresAt")]
  expires_at: Option<String>,
  /// GrantedScopes is a list of scopes that the subject authorized when asked for consent.
  #[serde(rename = "grantedScopes")]
  granted_scopes: Option<Vec<String>>,
  /// IssuedAt is the token creation time stamp.
  #[serde(rename = "issuedAt")]
  issued_at: Option<String>,
  /// Issuer is the id of the issuer, typically an hydra instance.
  #[serde(rename = "issuer")]
  issuer: Option<String>,
  /// Subject is the identity that authorized issuing the token, for example a user or an OAuth2 app. This is usually a uuid but you can choose a urn or some other id too.
  #[serde(rename = "subject")]
  subject: Option<String>
}

impl WardenTokenAccessRequestResponse {
  /// The warden access request (with token) response
  pub fn new() -> WardenTokenAccessRequestResponse {
    WardenTokenAccessRequestResponse {
      access_token_extra: None,
      allowed: None,
      client_id: None,
      expires_at: None,
      granted_scopes: None,
      issued_at: None,
      issuer: None,
      subject: None
    }
  }

  pub fn set_access_token_extra(&mut self, access_token_extra: ::std::collections::HashMap<String, Value>) {
    self.access_token_extra = Some(access_token_extra);
  }

  pub fn with_access_token_extra(mut self, access_token_extra: ::std::collections::HashMap<String, Value>) -> WardenTokenAccessRequestResponse {
    self.access_token_extra = Some(access_token_extra);
    self
  }

  pub fn access_token_extra(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.access_token_extra.as_ref()
  }

  pub fn reset_access_token_extra(&mut self) {
    self.access_token_extra = None;
  }

  pub fn set_allowed(&mut self, allowed: bool) {
    self.allowed = Some(allowed);
  }

  pub fn with_allowed(mut self, allowed: bool) -> WardenTokenAccessRequestResponse {
    self.allowed = Some(allowed);
    self
  }

  pub fn allowed(&self) -> Option<&bool> {
    self.allowed.as_ref()
  }

  pub fn reset_allowed(&mut self) {
    self.allowed = None;
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> WardenTokenAccessRequestResponse {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_expires_at(&mut self, expires_at: String) {
    self.expires_at = Some(expires_at);
  }

  pub fn with_expires_at(mut self, expires_at: String) -> WardenTokenAccessRequestResponse {
    self.expires_at = Some(expires_at);
    self
  }

  pub fn expires_at(&self) -> Option<&String> {
    self.expires_at.as_ref()
  }

  pub fn reset_expires_at(&mut self) {
    self.expires_at = None;
  }

  pub fn set_granted_scopes(&mut self, granted_scopes: Vec<String>) {
    self.granted_scopes = Some(granted_scopes);
  }

  pub fn with_granted_scopes(mut self, granted_scopes: Vec<String>) -> WardenTokenAccessRequestResponse {
    self.granted_scopes = Some(granted_scopes);
    self
  }

  pub fn granted_scopes(&self) -> Option<&Vec<String>> {
    self.granted_scopes.as_ref()
  }

  pub fn reset_granted_scopes(&mut self) {
    self.granted_scopes = None;
  }

  pub fn set_issued_at(&mut self, issued_at: String) {
    self.issued_at = Some(issued_at);
  }

  pub fn with_issued_at(mut self, issued_at: String) -> WardenTokenAccessRequestResponse {
    self.issued_at = Some(issued_at);
    self
  }

  pub fn issued_at(&self) -> Option<&String> {
    self.issued_at.as_ref()
  }

  pub fn reset_issued_at(&mut self) {
    self.issued_at = None;
  }

  pub fn set_issuer(&mut self, issuer: String) {
    self.issuer = Some(issuer);
  }

  pub fn with_issuer(mut self, issuer: String) -> WardenTokenAccessRequestResponse {
    self.issuer = Some(issuer);
    self
  }

  pub fn issuer(&self) -> Option<&String> {
    self.issuer.as_ref()
  }

  pub fn reset_issuer(&mut self) {
    self.issuer = None;
  }

  pub fn set_subject(&mut self, subject: String) {
    self.subject = Some(subject);
  }

  pub fn with_subject(mut self, subject: String) -> WardenTokenAccessRequestResponse {
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



