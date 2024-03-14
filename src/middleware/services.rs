use std::sync::Arc;
use actix_web::web;
use crate::passthrough_card::service::PassthroughCardService;
use crate::lithic::service::LithicService as LithicService;
use crate::charge::service::ChargeService;
use crate::user::dao::UserDao;
use crate::adyen::checkout::service::AdyenCheckoutService as AdyenChargeService;
use crate::category::dao::MccMappingDao;
use crate::credit_card_type::dao::{
    CreditCardDao,
};
use crate::footprint::service::FootprintService;
use crate::passthrough_card::dao::PassthroughCardDao;
use crate::rule::service::RuleService;
use crate::schema::registered_transactions::mcc;
use crate::ledger::service::LedgerService as LedgerEngine;
use crate::user::service::UserService;
use crate::wallet::{
    dao::{WalletDao, WalletCardAttemptDao},
    service::WalletService
};
use crate::webhooks::adyen_handler::AdyenHandler;
use crate::webhooks::lithic_handler::LithicHandler;

#[derive(Clone)]
pub struct Services {
    // TODO: no dao's should be present in services layer
    pub passthrough_card_service: Arc<PassthroughCardService>,
    //lithic: Arc<LithicService>,
    pub charge_service: Arc<ChargeService>,
    pub wallet_service: Arc<WalletService>,
    pub adyen_handler: Arc<AdyenHandler>,
    pub lithic_handler: Arc<LithicHandler>,
    pub user_dao: Arc<UserDao>,
    pub credit_card_dao: Arc<CreditCardDao>,
    pub rule_service: Arc<RuleService>,
    pub user_service: Arc<UserService>,
    pub footprint_service: Arc<FootprintService>
}

impl Services {
    #[tracing::instrument(skip_all)]
    pub fn new() -> Self {
        // TODO: these might need to be initialized in main
        let lithic_service = Arc::new(LithicService::new());
        let adyen_service = Arc::new(AdyenChargeService::new());
        let credit_card_dao = Arc::new(CreditCardDao::new());
        let wallet_dao = Arc::new(WalletDao::new());
        let wallet_card_attempt_dao = Arc::new(WalletCardAttemptDao::new());
        let footprint_service = Arc::new(FootprintService::new());
        let wallet_engine = Arc::new(WalletService::new_with_services(
            credit_card_dao.clone(),
            wallet_card_attempt_dao.clone(),
            wallet_dao.clone(),
            adyen_service.clone()
        ));
        let passthrough_card_dao = Arc::new(PassthroughCardDao::new());
        let user_dao = Arc::new(UserDao::new());
        let ledger = Arc::new(LedgerEngine::new());
        let charge_engine = Arc::new(ChargeService::new_with_service(
            adyen_service.clone(),
            passthrough_card_dao.clone(),
            user_dao.clone(),
            ledger.clone(),
            footprint_service.clone()
        ));
        let mcc_mapping_dao = Arc::new(MccMappingDao::new());
        let rule_engine = Arc::new(RuleService::new_with_services(
            mcc_mapping_dao.clone()
        ));
        let user_service = Arc::new(UserService::new_with_services(
            user_dao.clone(),
            footprint_service.clone()
        ));
        Self {
            passthrough_card_service: Arc::new(PassthroughCardService::new_with_services(
                lithic_service.clone(),
                passthrough_card_dao.clone()
            )),
            charge_service: Arc::new(ChargeService::new_with_service(
                adyen_service.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone(),
                ledger.clone(),
                footprint_service.clone()
            )),
            wallet_service: Arc::new(WalletService::new_with_services(
                credit_card_dao.clone(),
                wallet_card_attempt_dao.clone(),
                wallet_dao.clone(),
                adyen_service.clone()
            )),
            adyen_handler: Arc::new(AdyenHandler::new_with_service(
                wallet_engine.clone()
            )),
            lithic_handler: Arc::new(LithicHandler::new_with_services(
                charge_engine.clone(),
                rule_engine.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone()
            )),
            user_dao: user_dao.clone(),
            credit_card_dao: credit_card_dao.clone(),
            rule_service: rule_engine.clone(),
            user_service: user_service.clone(),
            footprint_service: footprint_service.clone()
        }
    }
}