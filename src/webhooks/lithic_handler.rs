use crate::adyen_service::checkout::service::AdyenChargeServiceTrait;

use crate::api_error::ApiError;
use crate::charge_engine::engine::Engine as ChargeEngine;
use crate::asa::request::AsaRequest;
use crate::rule_engine::engine::RuleEngine;
use crate::user::entity::User;
use crate::rule_engine::engine::RuleEngineTrait;

use crate::asa::response::{AsaResponse, AsaResponseResult};
use crate::passthrough_card::dao::{PassthroughCardDao, PassthroughCardDaoTrait};
use crate::passthrough_card::entity::PassthroughCard;
use crate::service_error::ServiceError;
use crate::transaction::engine::TransactionEngineTrait;
use crate::user::dao::{UserDao, UserDaoTrait};

pub struct LithicHandler {
    pub charge_engine: ChargeEngine,
    pub rule_engine: Box<dyn RuleEngineTrait>,
}

impl LithicHandler {
    pub fn new() -> Self {
        Self {
            charge_engine: ChargeEngine::new(),
            rule_engine: Box::new(RuleEngine::new()),
        }
    }

    #[cfg(test)]
    pub fn new_with_engines(
        charge_service: Box<dyn AdyenChargeServiceTrait>,
        passthrough_card_dao: Box<dyn PassthroughCardDaoTrait>,
        user_dao: Box<dyn UserDaoTrait>,
        ledger: Box<dyn TransactionEngineTrait>,
        rule_engine: Box<dyn RuleEngineTrait>
    ) -> Self {
        Self {
            charge_engine: ChargeEngine::new_with_service(
                charge_service,
                passthrough_card_dao,
                user_dao,
                ledger
            ),
            rule_engine: rule_engine,
        }
    }
    pub async fn handle(&self, request: AsaRequest) -> Result<AsaResponse, ApiError>{
        // TODO: do a reverse lookup based on the card token to get the user
        info!("Identifying user by card");
        println!("Identifying user by card");
        let card = request.card.clone().ok_or(ServiceError::new(400, "expect card".to_string()))?;
        let token = card.token.clone().ok_or(ServiceError::new(400, "expect token".to_string()))?;
        let user = User::find_by_internal_id(
            PassthroughCard::get_by_token(
                token.clone()
            )?.user_id
        )?;

        info!("Getting user cards for userId={}", user.id);
        println!("Getting user cards for userId={}", user.id);
        let cards = self.rule_engine.order_user_cards_for_request(
            request.clone(),
            &user
        )?;
        info!("Got {} cards for userId={}", cards.len(), user.id);
        println!("Got {} cards for userId={}", cards.len(), user.id);

        info!("Attempting to charge userId={}", user.id);
        println!("Attempting to charge userId={}", user.id);
        let (result, ledger) = self.charge_engine.charge_from_asa_request(
            &request,
            &cards,
        ).await?;
        println!("Done");
        info!("Charging success {:?} for userId={}", &result, user.id);
        println!("Charging success {:?} for userId={}", &result, user.id);
        Ok(
            AsaResponse {
                token: token,
                result: AsaResponseResult::from(result),
                avs_result: None,
                balance: None,
            }
        )
    }
}