use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;


#[derive(Deserialize)]
pub struct ApplicationConfiguration {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16
}