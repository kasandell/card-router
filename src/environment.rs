use lazy_static::lazy_static;
use crate::constant::env_key;
use std::env;

pub struct Environment {
    pub database_url: String,
    pub redis_url: String,
    pub adyen_api_key: String,
    pub adyen_merchant_account_name: String,
    pub lithic_api_key: String,
    pub mode: String,
    pub lithic_webhook_url: String
}


lazy_static! {
    pub static ref ENVIRONMENT: Environment = {
        Environment {
            database_url: env::var(env_key::DATABASE_URL).expect("should have database url"),
            redis_url: env::var(env_key::REDIS_URL).expect("should have database url"),
            adyen_api_key: env::var(env_key::ADYEN_API_KEY).expect("should have adyen api key"),
            adyen_merchant_account_name: env::var(env_key::ADYEN_MERCHANT_ACCOUNT_NAME).expect("should have adyen merchant account name"),
            lithic_api_key: env::var(env_key::LITHIC_API_KEY_NAME).expect("should have lithic api key"),
            mode: env::var(env_key::MODE_KEY).expect("should have mode"),
            lithic_webhook_url: env::var(env_key::LITHIC_WEBHOOK_URL_KEY).expect("should have lithic webhook url"),
        }
    };
}