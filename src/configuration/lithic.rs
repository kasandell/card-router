use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LithicConfiguration {
    pub api_key: Secret<String>,
    pub mode: String
}