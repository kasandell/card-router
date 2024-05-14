use secrecy::Secret;
use serde_with::serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AdyenConfiguration {
    pub merchant_account_name: String,
    pub api_key: Secret<String>
}