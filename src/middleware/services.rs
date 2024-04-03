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
    #[cfg(feature = "fake-footprint")]
    pub footprint_service: Arc<FakeFootprintService>,
    #[cfg(not(feature = "fake-footprint"))]
    pub footprint_service: Arc<FootprintService>,
}

impl Services {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        tracing::info!("Instantiating all services");
        // TODO: these might need to be initialized in main
        let lithic_service = Arc::new(LithicService::new());
        let credit_card_service = Arc::new(CreditCardService::new());
        #[cfg(feature = "fake-footprint")]
        let footprint_service = Arc::new(FakeFootprintService::new());
        #[cfg(not(feature = "fake-footprint"))]
        let footprint_service = Arc::new(FootprintService::new());
        let wallet_service = Arc::new(WalletService::new_with_services(
            credit_card_service.clone(),
            footprint_service.clone()
        ));
        let passthrough_card_service = Arc::new(PassthroughCardService::new());
        let ledger = Arc::new(LedgerEngine::new());
        let user_service = Arc::new(UserService::new_with_services(
            footprint_service.clone()
        ));
        let charge_service = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger.clone(),
            footprint_service.clone()
        ));
        let category_service = Arc::new(CategoryService::new());
        let rule_service = Arc::new(RuleService::new_with_services(
            category_service.clone(),
            wallet_service.clone()
        ));
        Self {
            passthrough_card_service: Arc::new(PassthroughCardService::new_with_services(
                lithic_service.clone(),
            )),
            charge_service: charge_service.clone(),
            wallet_service: wallet_service.clone(),
            lithic_handler: Arc::new(LithicHandler::new_with_services(
                charge_service.clone(),
                rule_service.clone(),
                passthrough_card_service.clone(),
                user_service.clone()
            )),
            user_service: user_service.clone(),
            credit_card_service: credit_card_service.clone(),
            rule_service: rule_service.clone(),
            footprint_service: footprint_service.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::middleware::services::Services;

    #[test]
    fn test_services_create() {
        let svc_middleware = Services::new();
    }
}