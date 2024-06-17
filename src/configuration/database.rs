use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pool_size: u32
}