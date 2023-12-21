use std::env;
use std::slice::RSplit;
use lithic_client::apis::card_api::{patch_card_by_token, post_cards};
use lithic_client::apis::configuration::{ApiKey, Configuration};
use lithic_client::models::patch_card_by_token_request::State as PatchState;
use lithic_client::models::post_cards_request::{
    State,
    Type
};
use lithic_client::models::{PostCardsRequest, Card, PatchCardByTokenRequest};
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


}

pub struct LithicService {
    configuration: Configuration
}

impl LithicService {
    pub fn new() -> Self {
        let mut cfg = Configuration::new();
        cfg.api_key = Some(ApiKey {
            prefix: None,
            key: env::var(env_key::LITHIC_API_KEY_NAME).expect("need api key")
        });
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
}