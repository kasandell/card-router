use std::sync::Arc;
use actix_web::web;
use crate::passthrough_card::service::PassthroughCardService as PassthroughCardEngine;
use crate::lithic::service::LithicService as LithicService;
use crate::charge::engine::Engine as ChargeEngine;
use crate::user::dao::UserDao;
use crate::adyen::checkout::service::ChargeService as AdyenChargeService;
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
    service::WalletService as WalletEngine
};
use crate::webhooks::adyen_handler::AdyenHandler;
use crate::webhooks::lithic_handler::LithicHandler;

#[derive(Clone)]
pub struct Services {
    // TODO: no dao's should be present in services layer
    pub passthrough_card_engine: Arc<PassthroughCardEngine>,
    //lithic: Arc<LithicService>,
    pub charge_engine: Arc<ChargeEngine>,
    pub wallet_engine: Arc<WalletEngine>,
    pub adyen_handler: Arc<AdyenHandler>,
    pub lithic_handler: Arc<LithicHandler>,
    pub user_dao: Arc<UserDao>,
    pub credit_card_dao: Arc<CreditCardDao>,
    pub rule_engine: Arc<RuleService>,
    pub user_service: Arc<UserService>,
    pub footprint_service: Arc<FootprintService>
}

impl Services {
    pub fn new() -> Self {
        // TODO: these might need to be initialized in main
        let lithic_service = Arc::new(LithicService::new());
        let adyen_service = Arc::new(AdyenChargeService::new());
        let credit_card_dao = Arc::new(CreditCardDao::new());
        let wallet_dao = Arc::new(WalletDao::new());
        let wallet_card_attempt_dao = Arc::new(WalletCardAttemptDao::new());
        let footprint_service = Arc::new(FootprintService::new());
        let wallet_engine = Arc::new(WalletEngine::new_with_services(
            credit_card_dao.clone(),
            wallet_card_attempt_dao.clone(),
            wallet_dao.clone(),
            adyen_service.clone()
        ));
        let passthrough_card_dao = Arc::new(PassthroughCardDao::new());
        let user_dao = Arc::new(UserDao::new());
        let ledger = Arc::new(LedgerEngine::new());
        let charge_engine = Arc::new(ChargeEngine::new_with_service(
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
            passthrough_card_engine: Arc::new(PassthroughCardEngine::new_with_service(lithic_service.clone())),
            charge_engine: Arc::new(ChargeEngine::new_with_service(
                adyen_service.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone(),
                ledger.clone(),
                footprint_service.clone()
            )),
            wallet_engine: Arc::new(WalletEngine::new_with_services(
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
            rule_engine: rule_engine.clone(),
            user_service: user_service.clone(),
            footprint_service: footprint_service.clone()
        }
    }
}