use std::sync::Arc;
use std::time::Instant;
use crate::adyen::checkout::service::AdyenChargeServiceTrait;

use crate::error::api_error::ApiError;
use crate::charge::service::ChargeService;
use crate::asa::request::AsaRequest;
use crate::rule::service::RuleService;
use crate::rule::service::RuleServiceTrait;
use crate::asa::response::{AsaResponse, AsaResponseResult};
use crate::error::error_type::ErrorType;
use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::passthrough_card::dao::{PassthroughCardDao, PassthroughCardDaoTrait};
use crate::error::service_error::ServiceError;
use crate::ledger::service::LedgerServiceTrait;
use crate::user::dao::{UserDao, UserDaoTrait};

pub struct LithicHandler {
    pub charge_service: Arc<ChargeService>,
    pub rule_service: Arc<dyn RuleServiceTrait>,
    pub passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
    pub user_dao: Arc<dyn UserDaoTrait>,
}

impl LithicHandler {
    #[tracing::instrument(skip_all)]
    pub fn new() -> Self {
        Self {
            charge_service: Arc::new(ChargeService::new()),
            rule_service: Arc::new(RuleService::new()),
            passthrough_card_dao: Arc::new(PassthroughCardDao::new()),
            user_dao: Arc::new(UserDao::new())
        }
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub fn new_with_engines(
        charge_service: Arc<dyn AdyenChargeServiceTrait>,
        passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
        user_dao: Arc<dyn UserDaoTrait>,
        ledger: Arc<dyn LedgerServiceTrait>,
        rule_engine: Arc<dyn RuleServiceTrait>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            charge_service: Arc::new(ChargeService::new_with_service(
                charge_service.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone(),
                ledger.clone(),
                footprint_service.clone()
            )),
            rule_service: rule_engine.clone(),
            passthrough_card_dao: passthrough_card_dao.clone(),
            user_dao: user_dao.clone()
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn new_with_services(
        charge_service: Arc<ChargeService>,
        rule_service: Arc<RuleService>,
        passthrough_card_dao: Arc<PassthroughCardDao>,
        user_dao: Arc<UserDao>,
    ) -> Self {
        Self {
            charge_service: charge_service,
            rule_service: rule_service,
            passthrough_card_dao,
            user_dao
        }
    }
    #[tracing::instrument(skip(self))]
    pub async fn handle(self: Arc<Self>, request: AsaRequest) -> Result<AsaResponse, ApiError>{
        // TODO: do a reverse lookup based on the card token to get the user
        tracing::info!("{:?}", &request);
        tracing::info!("Identifying user by card");
        let mut start = Instant::now();
        let card = request.card.clone().ok_or(ServiceError::new(ErrorType::BadRequest, "expect card"))?;
        tracing::info!("card from request took {:?}", start.elapsed());
        start = Instant::now();
        let token = card.token.clone().ok_or(ServiceError::new(ErrorType::BadRequest, "expect token"))?;
        tracing::info!("token from request took {:?}", start.elapsed());
        start = Instant::now();
        let passthrough_card = self.passthrough_card_dao.clone().get_by_token(&token).await?;
        tracing::info!("Find card took {:?}", start.elapsed());
        start = Instant::now();
        let user = self.user_dao.find_by_internal_id(passthrough_card.user_id).await?;
        //User::find_by_internal_id(passthrough_card.user_id).await?;
        tracing::info!("Find user took {:?}", start.elapsed());
        start = Instant::now();

        tracing::info!("Getting user cards for userId={}", user.id);
        let cards = self.rule_service.clone().order_user_cards_for_request(
            &request,
            &user
        ).await?;
        tracing::info!("Rule engine order cards took {:?}", start.elapsed());
        tracing::info!("Got {} cards for userId={}", cards.len(), user.id);
        tracing::info!("Attempting to charge userId={}", user.id);

        let (result, ledger) = self.charge_service.clone().charge_from_asa_request(
            &request,
            &cards,
            &passthrough_card,
            &user
        ).await?;

        Ok(
            AsaResponse {
                token,
                result: AsaResponseResult::from(result),
                avs_result: None,
                balance: None,
            }
        )
    }
}