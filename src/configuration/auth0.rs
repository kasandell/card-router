use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Auth0Configuration {
    pub authority: Secret<String>,
    pub audience: Secret<String>,
    pub domain: Secret<String>,
    pub client_origin_url: Secret<String>
}