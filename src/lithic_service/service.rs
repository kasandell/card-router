use std::env;
use std::time::Duration;
use lithic_client::apis::card_api::{patch_card_by_token, post_cards};
use lithic_client::apis::event_api::{create_event_subscription, delete_event_subscription};
use lithic_client::apis::configuration::{ApiKey, Configuration};
use lithic_client::models::event_subscription::EventSubscription;
use lithic_client::models::patch_card_by_token_request::State as PatchState;
use lithic_client::models::post_cards_request::{
    State,
    Type
};
use lithic_client::models::{PostCardsRequest, Card, PatchCardByTokenRequest, CreateEventSubscriptionRequest};
use uuid::Uuid;
use crate::constant::env_key;
use super::error::Error as LithicError;

#[mockall::automock]
pub trait LithicServiceTrait {
    fn create_card(
        &self,
        pin_str: String,
        idempotency_key: Uuid,
    ) -> Result<Card, LithicError>;

    fn close_card(
        &self,
        card_token: String
    ) -> Result<Card, LithicError>;

    fn activate_card(
        &self,
        card_token: String,
    ) -> Result<Card, LithicError>;
    fn pause_card(
        &self,
        card_token: String,
    ) -> Result<Card, LithicError>;

    fn patch_card(
        &self,
        card_token: String,
        state: Option<PatchState>,
        pin: Option<String>
    ) -> Result<Card, LithicError>;

    // TODO: this is not actually how we enroll. see https://github.com/lithic-com/asa-demo-python/blob/main/scripts/enroll.py
    fn register_webhook(&self, idempotency_key: String) -> Result<EventSubscription, LithicError>;

    fn deregister_webhook(&self, event_subscription_token: String) -> Result<(), LithicError>;
}

pub struct LithicService {
    configuration: Configuration
}

impl LithicService {
    pub fn new() -> Self {
        let mut cfg = Configuration::new();
        let base_path = match env::var(env_key::MODE_KEY).expect("need mode").as_str() {
            "production" => "https://api.lithic.com/v1".to_owned(),
            _ => "https://sandbox.lithic.com/v1".to_owned(),
        };
        cfg.base_path = base_path.clone();

        println!("base path");
        println!("{}", base_path);

        cfg.api_key = Some(ApiKey {
            prefix: None,
            key: env::var(env_key::LITHIC_API_KEY_NAME).expect("need api key")
        });

        println!("{:?}", cfg.api_key);

        println!("{:?}", env::var(env_key::LITHIC_API_KEY_NAME).expect("need api key"));

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

impl LithicServiceTrait for LithicService {
    fn create_card(
        &self,
        pin_str: String,
        idempotency_key: Uuid,
    ) -> Result<Card, LithicError> {
        Ok(
            futures::executor::block_on(async {
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
                }, None).await
            })?
        )
    }

    fn close_card(
        &self,
        card_token: String
    ) -> Result<Card, LithicError> {
        self.patch_card(
            card_token,
            Some(PatchState::Closed),
            None
        )

    }

    fn pause_card(
        &self,
        card_token: String,
    ) -> Result<Card, LithicError> {
        self.patch_card(
            card_token,
            Some(PatchState::Paused),
            None
        )
    }

    fn activate_card(
        &self,
        card_token: String,
    ) -> Result<Card, LithicError> {
        self.patch_card(
            card_token,
            Some(PatchState::Open),
            None
        )
    }

    fn patch_card(
        &self,
        card_token: String,
        state: Option<PatchState>,
        pin: Option<String>
    ) -> Result<Card, LithicError> {
        Ok(
            futures::executor::block_on( async {
                patch_card_by_token(
                    &self.configuration,
                    serde_json::to_value(card_token).expect("card should go to value"),
                    PatchCardByTokenRequest {
                        memo: None,
                        spend_limit: None,
                        spend_limit_duration: None,
                        auth_rule_token: None,
                        state: state,
                        pin: pin,
                        digital_card_art_token: None,
                    }

                ).await
            })?
        )
    }

    fn register_webhook(&self, idempotency_key: String) -> Result<EventSubscription, LithicError> {
        println!("registering");

        Ok(
            futures::executor::block_on(async {
                println!("i mean fuck bruh");
                println!("{:?}", &self.configuration);
                let res = create_event_subscription(
                    &self.configuration,
                    Some(serde_json::to_value(idempotency_key).expect("should work")),
                    Some(
                        CreateEventSubscriptionRequest {
                            description: Some("base event subscription".to_string()),
                            disabled: Some(false),
                            event_types: None,
                            url: env::var(crate::constant::env_key::LITHIC_WEBHOOK_URL_KEY).expect("Required config")
                        }
                    )
                ).await;

                println!("Made call");
                println!("{}", res.is_err());
                println!("{}", res.is_ok());

                res
            })?
        )
    }

    fn deregister_webhook(&self, event_subscription_token: String) -> Result<(), LithicError> {
        Ok(
            futures::executor::block_on(async {
                delete_event_subscription(
                    &self.configuration,
                    event_subscription_token.as_str(),
                    None
                ).await
            })?
        )
    }
}