use std::sync::Arc;
use actix_web::web;
use crate::lithic::service::LithicService as LithicService;
use crate::charge::service::ChargeService;
use crate::user::service::{UserService, UserServiceTrait};
use crate::adyen::checkout::service::AdyenCheckoutService as AdyenChargeService;
use crate::category::service::{CategoryService, CategoryServiceTrait};
use crate::credit_card_type::service::{
    CreditCardServiceTrait,
    CreditCardService
};
use crate::footprint::service::{FakeFootprintService, FootprintService};
use crate::passthrough_card::service::{PassthroughCardService, PassthroughCardServiceTrait};
use crate::rule::service::RuleService;
use crate::ledger::service::LedgerService as LedgerEngine;
use crate::wallet::{
    dao::{WalletDao, WalletCardAttemptDao},
    service::WalletService
};
use crate::webhooks::lithic_handler::LithicHandler;

#[derive(Clone)]
pub struct Services {
    // TODO: no dao's should be present in services layer
    pub passthrough_card_service: Arc<PassthroughCardService>,
    pub charge_service: Arc<ChargeService>,
    pub wallet_service: Arc<WalletService>,
    pub lithic_handler: Arc<LithicHandler>,
    pub user_service: Arc<UserService>,
    pub credit_card_service: Arc<CreditCardService>,
    pub rule_service: Arc<RuleService>,
    pub footprint_service: Arc<FakeFootprintService>
}

impl Services {
    #[tracing::instrument(skip_all)]
    pub fn new() -> Self {
        // TODO: these might need to be initialized in main
        let lithic_service = Arc::new(LithicService::new());
        let adyen_service = Arc::new(AdyenChargeService::new());
        let credit_card_service = Arc::new(CreditCardService::new());
        let wallet_dao = Arc::new(WalletDao::new());
        let wallet_card_attempt_dao = Arc::new(WalletCardAttemptDao::new());
        let footprint_service = Arc::new(FakeFootprintService::new());
        let wallet_engine = Arc::new(WalletService::new_with_services(
            credit_card_dao.clone(),
            wallet_card_attempt_dao.clone(),
            wallet_dao.clone(),
            adyen_service.clone(),
            footprint_service.clone()
        ));
        let passthrough_card_service = Arc::new(PassthroughCardService::new());
        let user_service = Arc::new(UserService::new());
        let ledger = Arc::new(LedgerEngine::new());
        let charge_engine = Arc::new(ChargeService::new_with_service(
            passthrough_card_service.clone(),
            user_service.clone(),
            ledger.clone(),
            footprint_service.clone()
        ));
        let category_service = Arc::new(CategoryService::new());
        let rule_engine = Arc::new(RuleService::new_with_services(
            mcc_mapping_dao.clone()
        ));
        let user_service = Arc::new(UserService::new_with_services(
            footprint_service.clone()
        ));
        Self {
            passthrough_card_service: Arc::new(PassthroughCardService::new_with_services(
                lithic_service.clone(),
            )),
            charge_service: charge_engine.clone(),
            wallet_service: wallet_engine.clone(),
            lithic_handler: Arc::new(LithicHandler::new_with_services(
                charge_engine.clone(),
                rule_engine.clone(),
                passthrough_card_service.clone(),
                user_service.clone()
            )),
            user_service: user_service.clone(),
            credit_card_service: credit_card_service.clone(),
            rule_service: rule_engine.clone(),
            user_service: user_service.clone(),
            footprint_service: footprint_service.clone()
        }
    }
}