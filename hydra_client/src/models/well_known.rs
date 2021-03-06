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
pub struct WellKnown {
  /// URL of the OP's OAuth 2.0 Authorization Endpoint
  #[serde(rename = "authorization_endpoint")]
  authorization_endpoint: String,
  /// JSON array containing a list of the Claim Names of the Claims that the OpenID Provider MAY be able to supply values for. Note that for privacy or other reasons, this might not be an exhaustive list.
  #[serde(rename = "claims_supported")]
  claims_supported: Option<Vec<String>>,
  /// JSON array containing a list of the JWS signing algorithms (alg values) supported by the OP for the ID Token to encode the Claims in a JWT.
  #[serde(rename = "id_token_signing_alg_values_supported")]
  id_token_signing_alg_values_supported: Vec<String>,
  /// URL using the https scheme with no query or fragment component that the OP asserts as its Issuer Identifier. If Issuer discovery is supported , this value MUST be identical to the issuer value returned by WebFinger. This also MUST be identical to the iss Claim value in ID Tokens issued from this Issuer.
  #[serde(rename = "issuer")]
  issuer: String,
  /// URL of the OP's JSON Web Key Set [JWK] document. This contains the signing key(s) the RP uses to validate signatures from the OP. The JWK Set MAY also contain the Server's encryption key(s), which are used by RPs to encrypt requests to the Server. When both signing and encryption keys are made available, a use (Key Use) parameter value is REQUIRED for all keys in the referenced JWK Set to indicate each key's intended usage. Although some algorithms allow the same key to be used for both signatures and encryption, doing so is NOT RECOMMENDED, as it is less secure. The JWK x5c parameter MAY be used to provide X.509 representations of keys provided. When used, the bare key values MUST still be present and MUST match those in the certificate.
  #[serde(rename = "jwks_uri")]
  jwks_uri: String,
  /// JSON array containing a list of the OAuth 2.0 response_type values that this OP supports. Dynamic OpenID Providers MUST support the code, id_token, and the token id_token Response Type values.
  #[serde(rename = "response_types_supported")]
  response_types_supported: Vec<String>,
  /// SON array containing a list of the OAuth 2.0 [RFC6749] scope values that this server supports. The server MUST support the openid scope value. Servers MAY choose not to advertise some supported scope values even when this parameter is used
  #[serde(rename = "scopes_supported")]
  scopes_supported: Option<Vec<String>>,
  /// JSON array containing a list of the Subject Identifier types that this OP supports. Valid types include pairwise and public.
  #[serde(rename = "subject_types_supported")]
  subject_types_supported: Vec<String>,
  /// URL of the OP's OAuth 2.0 Token Endpoint
  #[serde(rename = "token_endpoint")]
  token_endpoint: String,
  /// JSON array containing a list of Client Authentication methods supported by this Token Endpoint. The options are client_secret_post, client_secret_basic, client_secret_jwt, and private_key_jwt, as described in Section 9 of OpenID Connect Core 1.0
  #[serde(rename = "token_endpoint_auth_methods_supported")]
  token_endpoint_auth_methods_supported: Option<Vec<String>>,
  /// URL of the OP's UserInfo Endpoint.
  #[serde(rename = "userinfo_endpoint")]
  userinfo_endpoint: Option<String>
}

impl WellKnown {
  pub fn new(authorization_endpoint: String, id_token_signing_alg_values_supported: Vec<String>, issuer: String, jwks_uri: String, response_types_supported: Vec<String>, subject_types_supported: Vec<String>, token_endpoint: String) -> WellKnown {
    WellKnown {
      authorization_endpoint: authorization_endpoint,
      claims_supported: None,
      id_token_signing_alg_values_supported: id_token_signing_alg_values_supported,
      issuer: issuer,
      jwks_uri: jwks_uri,
      response_types_supported: response_types_supported,
      scopes_supported: None,
      subject_types_supported: subject_types_supported,
      token_endpoint: token_endpoint,
      token_endpoint_auth_methods_supported: None,
      userinfo_endpoint: None
    }
  }

  pub fn set_authorization_endpoint(&mut self, authorization_endpoint: String) {
    self.authorization_endpoint = authorization_endpoint;
  }

  pub fn with_authorization_endpoint(mut self, authorization_endpoint: String) -> WellKnown {
    self.authorization_endpoint = authorization_endpoint;
    self
  }

  pub fn authorization_endpoint(&self) -> &String {
    &self.authorization_endpoint
  }


  pub fn set_claims_supported(&mut self, claims_supported: Vec<String>) {
    self.claims_supported = Some(claims_supported);
  }

  pub fn with_claims_supported(mut self, claims_supported: Vec<String>) -> WellKnown {
    self.claims_supported = Some(claims_supported);
    self
  }

  pub fn claims_supported(&self) -> Option<&Vec<String>> {
    self.claims_supported.as_ref()
  }

  pub fn reset_claims_supported(&mut self) {
    self.claims_supported = None;
  }

  pub fn set_id_token_signing_alg_values_supported(&mut self, id_token_signing_alg_values_supported: Vec<String>) {
    self.id_token_signing_alg_values_supported = id_token_signing_alg_values_supported;
  }

  pub fn with_id_token_signing_alg_values_supported(mut self, id_token_signing_alg_values_supported: Vec<String>) -> WellKnown {
    self.id_token_signing_alg_values_supported = id_token_signing_alg_values_supported;
    self
  }

  pub fn id_token_signing_alg_values_supported(&self) -> &Vec<String> {
    &self.id_token_signing_alg_values_supported
  }


  pub fn set_issuer(&mut self, issuer: String) {
    self.issuer = issuer;
  }

  pub fn with_issuer(mut self, issuer: String) -> WellKnown {
    self.issuer = issuer;
    self
  }

  pub fn issuer(&self) -> &String {
    &self.issuer
  }


  pub fn set_jwks_uri(&mut self, jwks_uri: String) {
    self.jwks_uri = jwks_uri;
  }

  pub fn with_jwks_uri(mut self, jwks_uri: String) -> WellKnown {
    self.jwks_uri = jwks_uri;
    self
  }

  pub fn jwks_uri(&self) -> &String {
    &self.jwks_uri
  }


  pub fn set_response_types_supported(&mut self, response_types_supported: Vec<String>) {
    self.response_types_supported = response_types_supported;
  }

  pub fn with_response_types_supported(mut self, response_types_supported: Vec<String>) -> WellKnown {
    self.response_types_supported = response_types_supported;
    self
  }

  pub fn response_types_supported(&self) -> &Vec<String> {
    &self.response_types_supported
  }


  pub fn set_scopes_supported(&mut self, scopes_supported: Vec<String>) {
    self.scopes_supported = Some(scopes_supported);
  }

  pub fn with_scopes_supported(mut self, scopes_supported: Vec<String>) -> WellKnown {
    self.scopes_supported = Some(scopes_supported);
    self
  }

  pub fn scopes_supported(&self) -> Option<&Vec<String>> {
    self.scopes_supported.as_ref()
  }

  pub fn reset_scopes_supported(&mut self) {
    self.scopes_supported = None;
  }

  pub fn set_subject_types_supported(&mut self, subject_types_supported: Vec<String>) {
    self.subject_types_supported = subject_types_supported;
  }

  pub fn with_subject_types_supported(mut self, subject_types_supported: Vec<String>) -> WellKnown {
    self.subject_types_supported = subject_types_supported;
    self
  }

  pub fn subject_types_supported(&self) -> &Vec<String> {
    &self.subject_types_supported
  }


  pub fn set_token_endpoint(&mut self, token_endpoint: String) {
    self.token_endpoint = token_endpoint;
  }

  pub fn with_token_endpoint(mut self, token_endpoint: String) -> WellKnown {
    self.token_endpoint = token_endpoint;
    self
  }

  pub fn token_endpoint(&self) -> &String {
    &self.token_endpoint
  }


  pub fn set_token_endpoint_auth_methods_supported(&mut self, token_endpoint_auth_methods_supported: Vec<String>) {
    self.token_endpoint_auth_methods_supported = Some(token_endpoint_auth_methods_supported);
  }

  pub fn with_token_endpoint_auth_methods_supported(mut self, token_endpoint_auth_methods_supported: Vec<String>) -> WellKnown {
    self.token_endpoint_auth_methods_supported = Some(token_endpoint_auth_methods_supported);
    self
  }

  pub fn token_endpoint_auth_methods_supported(&self) -> Option<&Vec<String>> {
    self.token_endpoint_auth_methods_supported.as_ref()
  }

  pub fn reset_token_endpoint_auth_methods_supported(&mut self) {
    self.token_endpoint_auth_methods_supported = None;
  }

  pub fn set_userinfo_endpoint(&mut self, userinfo_endpoint: String) {
    self.userinfo_endpoint = Some(userinfo_endpoint);
  }

  pub fn with_userinfo_endpoint(mut self, userinfo_endpoint: String) -> WellKnown {
    self.userinfo_endpoint = Some(userinfo_endpoint);
    self
  }

  pub fn userinfo_endpoint(&self) -> Option<&String> {
    self.userinfo_endpoint.as_ref()
  }

  pub fn reset_userinfo_endpoint(&mut self) {
    self.userinfo_endpoint = None;
  }

}



