use std::sync::Arc;
use std::time::Instant;
use crate::adyen::checkout::service::AdyenChargeServiceTrait;

use crate::charge::service::{ChargeService, ChargeServiceTrait};
use crate::asa::request::AsaRequest;
use crate::rule::service::RuleService;
use crate::rule::service::RuleServiceTrait;
use crate::asa::response::{AsaResponse, AsaResponseResult};

use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::ledger::service::LedgerServiceTrait;
use crate::passthrough_card::service::PassthroughCardServiceTrait;
use crate::user::service::UserServiceTrait;
use super::error::LithicHandlerError;

pub struct LithicHandler {
    charge_service: Arc<dyn ChargeServiceTrait>,
    rule_service: Arc<dyn RuleServiceTrait>,
    passthrough_card_service: Arc<dyn PassthroughCardServiceTrait>,
    user_service: Arc<dyn UserServiceTrait>,
}

impl LithicHandler {

    #[tracing::instrument(skip_all)]
    pub fn new_with_services(
        charge_service: Arc<dyn ChargeServiceTrait>,
        rule_service: Arc<RuleService>,
        passthrough_card_service: Arc<dyn PassthroughCardServiceTrait>,
        user_service: Arc<dyn UserServiceTrait>,
    ) -> Self {
        Self {
            charge_service,
            rule_service,
            passthrough_card_service,
            user_service
        }
    }
    #[tracing::instrument(skip(self))]
    pub async fn handle(self: Arc<Self>, request: AsaRequest) -> Result<AsaResponse, LithicHandlerError>{
        // TODO: do a reverse lookup based on the card token to get the user
        tracing::warn!("runtime: {:?}, task: {:?}", tokio::runtime::Handle::current().id(), tokio::task::id());
        tracing::info!("Identifying user by card");
        let card = request.card.clone().ok_or(
            LithicHandlerError::Unexpected("expect card in request".into())
        )?;
        let token = card.token.clone().ok_or(
            LithicHandlerError::Unexpected("expect token on card".into())
        )?;
        let passthrough_card = self.passthrough_card_service.clone().get_by_token(&token).await
            .map_err(|e| LithicHandlerError::Unexpected(e.into()))?;
        let user = self.user_service.clone().find_by_internal_id(passthrough_card.user_id).await
            .map_err(|e| LithicHandlerError::Unexpected(e.into()))?;

        tracing::info!("Getting user cards for userId={}", user.id);
        let cards = self.rule_service.clone().order_user_cards_for_request(
            &request,
            &user
        ).await.map_err(|e| LithicHandlerError::Unexpected(e.into()))?;
        tracing::info!("Got {} cards for userId={}", cards.len(), user.id);
        tracing::info!("Attempting to charge userId={}", user.id);

        let (result, ledger) = self.charge_service.clone().charge_from_asa_request(
            &request,
            &cards,
            &passthrough_card,
            &user
        ).await.map_err(|e| LithicHandlerError::Unexpected(e.into()))?;

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