use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use crate::adyen::checkout::service::AdyenChargeServiceTrait;

use crate::error::api_error::ApiError;
use crate::charge::engine::Engine as ChargeEngine;
use crate::asa::request::AsaRequest;
use crate::rule::service::RuleService;
use crate::user::entity::User;
use crate::rule::service::RuleServiceTrait;

use crate::asa::response::{AsaResponse, AsaResponseResult};
use crate::error::error_type::ErrorType;
use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::passthrough_card::dao::{PassthroughCardDao, PassthroughCardDaoTrait};
use crate::passthrough_card::entity::PassthroughCard;
use crate::error::service_error::ServiceError;
use crate::ledger::service::LedgerServiceTrait;
use crate::user::dao::{UserDao, UserDaoTrait};

pub struct LithicHandler {
    pub charge_engine: Arc<ChargeEngine>,
    pub rule_engine: Arc<dyn RuleServiceTrait>,
    pub passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
    pub user_dao: Arc<dyn UserDaoTrait>,
}

impl LithicHandler {
    pub fn new() -> Self {
        Self {
            charge_engine: Arc::new(ChargeEngine::new()),
            rule_engine: Arc::new(RuleService::new()),
            passthrough_card_dao: Arc::new(PassthroughCardDao::new()),
            user_dao: Arc::new(UserDao::new())
        }
    }

    #[cfg(test)]
    pub fn new_with_engines(
        charge_service: Arc<dyn AdyenChargeServiceTrait>,
        passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
        user_dao: Arc<dyn UserDaoTrait>,
        ledger: Arc<dyn LedgerServiceTrait>,
        rule_engine: Arc<dyn RuleServiceTrait>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            charge_engine: Arc::new(ChargeEngine::new_with_service(
                charge_service.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone(),
                ledger.clone(),
                footprint_service.clone()
            )),
            rule_engine: rule_engine.clone(),
            passthrough_card_dao: passthrough_card_dao.clone(),
            user_dao: user_dao.clone()
        }
    }

    pub fn new_with_services(
        charge_engine: Arc<ChargeEngine>,
        rule_engine: Arc<RuleService>,
        passthrough_card_dao: Arc<PassthroughCardDao>,
        user_dao: Arc<UserDao>,
    ) -> Self {
        Self {
            charge_engine,
            rule_engine,
            passthrough_card_dao,
            user_dao
        }
    }
    pub async fn handle(self: Arc<Self>, request: AsaRequest) -> Result<AsaResponse, ApiError>{
        // TODO: do a reverse lookup based on the card token to get the user
        println!("{:?}", &request);
        info!("Identifying user by card");
        println!("Identifying user by card");
        let mut start = Instant::now();
        let card = request.card.clone().ok_or(ServiceError::new(ErrorType::BadRequest, "expect card"))?;
        println!("card from request took {:?}", start.elapsed());
        start = Instant::now();
        let token = card.token.clone().ok_or(ServiceError::new(ErrorType::BadRequest, "expect token"))?;
        println!("token from request took {:?}", start.elapsed());
        start = Instant::now();
        let passthrough_card = self.passthrough_card_dao.clone().get_by_token(&token).await?;
        println!("Find card took {:?}", start.elapsed());
        start = Instant::now();
        let user = self.user_dao.find_by_internal_id(passthrough_card.user_id).await?;
        //User::find_by_internal_id(passthrough_card.user_id).await?;
        println!("Find user took {:?}", start.elapsed());
        start = Instant::now();

        info!("Getting user cards for userId={}", user.id);
        println!("Getting user cards for userId={}", user.id);
        let cards = self.rule_engine.clone().order_user_cards_for_request(
            &request,
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
            &passthrough_card,
            &user
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