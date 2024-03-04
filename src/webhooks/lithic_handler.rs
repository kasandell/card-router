use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
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
    pub charge_engine: Arc<ChargeEngine>,
    pub rule_engine: Arc<dyn RuleEngineTrait>,
}

impl LithicHandler {
    pub fn new() -> Self {
        Self {
            charge_engine: Arc::new(ChargeEngine::new()),
            rule_engine: Arc::new(RuleEngine::new()),
        }
    }

    #[cfg(test)]
    pub fn new_with_engines(
        charge_service: Arc<dyn AdyenChargeServiceTrait>,
        passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
        user_dao: Arc<dyn UserDaoTrait>,
        ledger: Arc<dyn TransactionEngineTrait>,
        rule_engine: Arc<dyn RuleEngineTrait>
    ) -> Self {
        Self {
            charge_engine: Arc::new(ChargeEngine::new_with_service(
                charge_service.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone(),
                ledger.clone()
            )),
            rule_engine: rule_engine.clone(),
        }
    }

    pub fn new_with_services(
        charge_engine: Arc<ChargeEngine>,
        rule_engine: Arc<RuleEngine>
    ) -> Self {
        Self {
            charge_engine,
            rule_engine
        }
    }
    pub async fn handle(self: Arc<Self>, request: AsaRequest) -> Result<AsaResponse, ApiError>{
        // TODO: do a reverse lookup based on the card token to get the user
        println!("{:?}", &request);
        info!("Identifying user by card");
        println!("Identifying user by card");
        let mut start = Instant::now();
        let card = request.card.clone().ok_or(ServiceError::new(400, "expect card".to_string()))?;
        let token = card.token.clone().ok_or(ServiceError::new(400, "expect token".to_string()))?;
        let user = User::find_by_internal_id(
            PassthroughCard::get_by_token(
                token.clone()
            ).await?.user_id
        ).await?;
        println!("Find user, card, token took {:?}", start.elapsed());
        start = Instant::now();

        info!("Getting user cards for userId={}", user.id);
        println!("Getting user cards for userId={}", user.id);
        let cards = self.rule_engine.clone().order_user_cards_for_request(
            request.clone(),
            &user
        ).await?;
        println!("Rule engine order cards took {:?}", start.elapsed());

        info!("Got {} cards for userId={}", cards.len(), user.id);
        println!("Got {} cards for userId={}", cards.len(), user.id);

        info!("Attempting to charge userId={}", user.id);
        println!("Attempting to charge userId={}", user.id);

        start = Instant::now();
        let (result, ledger) = self.charge_engine.clone().charge_from_asa_request(
            &request,
            &cards,
        ).await?;
        println!("Charge engine from asa request took {:?}", start.elapsed());
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