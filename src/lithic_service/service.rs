use std::env;
use std::time::Duration;
use lithic_client::apis::card_api::{patch_card_by_token, post_cards};
use lithic_client::apis::event_api::{create_event_subscription, delete_event_subscription};
use lithic_client::apis::configuration::{ApiKey, Configuration};
use lithic_client::models::event_subscription::EventSubscription;
use lithic_client::models::patch_card_by_token_request::{SpendLimitDuration, State as PatchState};
use lithic_client::models::post_cards_request::{
    State,
    Type
};
use lithic_client::models::{PostCardsRequest, Card, PatchCardByTokenRequest, CreateEventSubscriptionRequest};
use uuid::Uuid;
use crate::constant::env_key;
use super::error::Error as LithicError;
use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose;
use serde_json::json;
use crate::environment::ENVIRONMENT;
use crate::passthrough_card::crypto::encrypt_pin;


#[mockall::automock]
#[async_trait]
pub trait LithicServiceTrait {
    async fn create_card<'a>(
        &self,
        pin_str: &'a str,
        idempotency_key: &'a Uuid,
    ) -> Result<Card, LithicError>;

    async fn close_card(
        &self,
        card_token: &str,
    ) -> Result<Card, LithicError>;

    async fn activate_card(
        &self,
        card_token: &str,
    ) -> Result<Card, LithicError>;
    async fn pause_card(
        &self,
        card_token: &str,
    ) -> Result<Card, LithicError>;

    async fn patch_card<'a>(
        &self,
        card_token: &'a str,
        state: Option<&'a PatchState>,
        pin: Option<&'a str>
    ) -> Result<Card, LithicError>;

    // TODO: this is not actually how we enroll. see https://github.com/lithic-com/asa-demo-python/blob/main/scripts/enroll.py
    async fn register_webhook(&self, idempotency_key: &str) -> Result<EventSubscription, LithicError>;

    async fn deregister_webhook(&self, event_subscription_token: &str) -> Result<(), LithicError>;
}

pub struct LithicService {
    configuration: Configuration
}

impl LithicService {
    pub fn new() -> Self {
        let mut cfg = Configuration::new();
        let base_path = match ENVIRONMENT.mode.as_str() {
            "production" => "https://api.lithic.com/v1".to_owned(),
            _ => "https://sandbox.lithic.com/v1".to_owned(),
        };
        cfg.base_path = base_path.clone();

        cfg.api_key = Some(ApiKey {
            prefix: None,
            key: ENVIRONMENT.lithic_api_key.clone(),
        });

        let mut client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .connection_verbose(true)
            .build().expect("Fuck this");
        cfg.client = client;

        //cfg.base_path = base_path;
        LithicService {
            configuration: cfg
        }
    }
}

#[async_trait]
impl LithicServiceTrait for LithicService {
    async fn create_card<'a>(
        &self,
        pin_str: &'a str,
        idempotency_key: &'a Uuid,
    ) -> Result<Card, LithicError> {
        Ok(
            post_cards(&self.configuration, PostCardsRequest {
                account_token: None, // might need
                card_program_token: None,
                exp_month: None,
                exp_year: None,
                memo: None,
                spend_limit: None,
                spend_limit_duration: None,
                state: Some(State::Open),
                r#type: Type::Virtual,
                pin: None,
                digital_card_art_token: None,
                product_id: None,
                shipping_address: None,
                shipping_method: None,
                carrier: None,
            }, None).await?
        )
    }

    async fn close_card(
        &self,
        card_token: &str
    ) -> Result<Card, LithicError> {
        self.patch_card(
            card_token,
            Some(&PatchState::Closed),
            None
        ).await

    }

    async fn pause_card(
        &self,
        card_token: &str,
    ) -> Result<Card, LithicError> {
        self.patch_card(
            card_token,
            Some(&PatchState::Paused),
            None
        ).await
    }

    async fn activate_card(
        &self,
        card_token: &str,
    ) -> Result<Card, LithicError> {
        self.patch_card(
            card_token,
            Some(&PatchState::Open),
            None
        ).await
    }

    async fn patch_card<'a>(
        &self,
        card_token: &'a str,
        state: Option<&'a PatchState>,
        pin: Option<&'a str>
    ) -> Result<Card, LithicError> {
        Ok(
            patch_card_by_token(
                &self.configuration,
                serde_json::Value::String(card_token.to_string()),
                PatchCardByTokenRequest {
                    memo: None,
                    spend_limit: None,
                    spend_limit_duration: Some(SpendLimitDuration::Forever),
                    auth_rule_token: None,
                    state: state.copied(),
                    //Some(encrypt_pin("1234".to_string())),
                    pin: None,
                    digital_card_art_token: None,
                }
            ).await?
        )
    }

    async fn register_webhook(&self, idempotency_key: &str) -> Result<EventSubscription, LithicError> {
        Ok(
            create_event_subscription(
                &self.configuration,
                Some(serde_json::to_value(idempotency_key).expect("should work")),
                Some(
                    CreateEventSubscriptionRequest {
                        description: Some("base event subscription".to_string()),
                        disabled: Some(false),
                        event_types: None,
                        url: ENVIRONMENT.lithic_webhook_url.clone()
                    }
                )
            ).await?
        )
    }

    async fn deregister_webhook(&self, event_subscription_token: &str) -> Result<(), LithicError> {
        Ok(
            delete_event_subscription(
                &self.configuration,
                event_subscription_token,
                None
            ).await?
        )
    }
}