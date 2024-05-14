use serde::Deserialize;

#[derive(Deserialize)]
pub struct OtelConfiguration {
    pub otlp_endpoint: String,
    pub otlp_protocol: String,
    pub honeycomb_team: String,
    pub service_name: String,
}