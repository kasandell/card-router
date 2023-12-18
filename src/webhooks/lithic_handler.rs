use crate::adyen_service::checkout::service::AdyenChargeServiceTrait;

use crate::api_error::ApiError;
use crate::charge_engine::engine::Engine as ChargeEngine;
use crate::asa::request::AsaRequest;
use crate::rule_engine::engine::RuleEngine;
use crate::user::entity::User;
use crate::rule_engine::engine::RuleEngineTrait;

use crate::asa::response::{AsaResponse, AsaResponseResult};

pub struct LithicHandler {
    pub charge_engine: ChargeEngine,
    pub rule_engine: Box<dyn RuleEngineTrait>,
}

impl LithicHandler {
    pub fn new() -> Self {
        Self {
            charge_engine: ChargeEngine::new(),
            rule_engine: Box::new(RuleEngine::new())
        }
    }

    #[cfg(test)]
    pub fn new_with_engines(
        charge_service: Box<dyn AdyenChargeServiceTrait>,
        rule_engine: Box<dyn RuleEngineTrait>
    ) -> Self {
        Self {
            charge_engine: ChargeEngine::new_with_service(charge_service),
            rule_engine: rule_engine
        }
    }
    pub async fn handle(&self, request: AsaRequest) -> Result<AsaResponse, ApiError>{
        // TODO: do a reverse lookup based on the card token to get the user
        info!("Identifying user by card");
        let user = User {
            id: 1,
            public_id: Default::default(),
            email: "".to_string(),
            password: "".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        info!("Getting user cards for userId={}", user.id);
        let cards = self.rule_engine.order_user_cards_for_request(
            request.clone(),
            &user
        )?;
        info!("Got {} cards for userId={}", cards.len(), user.id);

        info!("Attempting to charge userId={}", user.id);
        let result = self.charge_engine.charge_wallet(
            &user,
            &cards,
            request.amount,
            &request.merchant.mcc,
            &request.merchant.descriptor
        ).await?;
        info!("Charging success {} for userId={}", result, user.id);
        Ok(
            AsaResponse {
                token: request.token,
                result: if result {AsaResponseResult::Approved} else {AsaResponseResult::UnauthorizedMerchant},
                avs_result: None,
                balance: None,
            }
        )
    }
}