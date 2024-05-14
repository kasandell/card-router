use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FootprintConfiguration {
    pub secret_key: Secret<String>
}