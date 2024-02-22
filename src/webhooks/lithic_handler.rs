use crate::adyen_service::checkout::service::AdyenChargeServiceTrait;

use crate::api_error::ApiError;
use crate::charge_engine::engine::Engine as ChargeEngine;
use crate::asa::request::AsaRequest;
use crate::rule_engine::engine::RuleEngine;
use crate::user::entity::User;
use crate::rule_engine::engine::RuleEngineTrait;

use crate::asa::response::{AsaResponse, AsaResponseResult};
use crate::passthrough_card::entity::PassthroughCard;

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
        let user = User::find_by_internal_id(
            PassthroughCard::get_by_token(request.token.clone())?.user_id
        )?;

        info!("Getting user cards for userId={}", user.id);
        let cards = self.rule_engine.order_user_cards_for_request(
            request.clone(),
            &user
        )?;
        info!("Got {} cards for userId={}", cards.len(), user.id);

        info!("Attempting to charge userId={}", user.id);
        let (result, ledger) = self.charge_engine.charge_from_asa_request(
            &request,
            &cards,
        ).await?;
        info!("Charging success {} for userId={}", result, user.id);
        Ok(
            AsaResponse {
                token: request.token,
                result: AsaResponseResult::from(result),
                avs_result: None,
                balance: None,
            }
        )
    }
}