use std::sync::Arc;
use actix_web::web;
use crate::passthrough_card::engine::Engine as PassthroughCardEngine;
use crate::lithic_service::service::LithicService as LithicService;
use crate::charge_engine::engine::Engine as ChargeEngine;
use crate::user::dao::UserDao;
use crate::adyen_service::checkout::service::ChargeService as AdyenChargeService;
use crate::category::dao::MccMappingDao;
use crate::credit_card_type::dao::{
    CreditCardDao,
};
use crate::passthrough_card::dao::PassthroughCardDao;
use crate::rule_engine::engine::RuleEngine;
use crate::schema::registered_transactions::mcc;
use crate::transaction::engine::Engine as LedgerEngine;
use crate::wallet::{
    dao::{WalletDao, WalletCardAttemptDao},
    engine::Engine as WalletEngine
};
use crate::webhooks::adyen_handler::AdyenHandler;
use crate::webhooks::lithic_handler::LithicHandler;

#[derive(Clone)]
pub struct Services {
    // TODO: no dao's should be present in services layer
    pub passthrough_card_engine: Arc<PassthroughCardEngine>,
    //lithic_service: Arc<LithicService>,
    pub charge_engine: Arc<ChargeEngine>,
    pub wallet_engine: Arc<WalletEngine>,
    pub adyen_handler: Arc<AdyenHandler>,
    pub lithic_handler: Arc<LithicHandler>,
    pub user_dao: Arc<UserDao>,
    pub credit_card_dao: Arc<CreditCardDao>,
    pub rule_engine: Arc<RuleEngine>,
}

impl Services {
    pub fn new() -> Self {
        // TODO: these might need to be initialized in main
        let lithic_service = Arc::new(LithicService::new());
        let adyen_service = Arc::new(AdyenChargeService::new());
        let credit_card_dao = Arc::new(CreditCardDao::new());
        let wallet_dao = Arc::new(WalletDao::new());
        let wallet_card_attempt_dao = Arc::new(WalletCardAttemptDao::new());
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
            ledger.clone()
        ));
        let mcc_mapping_dao = Arc::new(MccMappingDao::new());
        let rule_engine = Arc::new(RuleEngine::new_with_services(
            mcc_mapping_dao.clone()
        ));
        Self {
            passthrough_card_engine: Arc::new(PassthroughCardEngine::new_with_service(lithic_service.clone())),
            charge_engine: Arc::new(ChargeEngine::new_with_service(
                adyen_service.clone(),
                passthrough_card_dao.clone(),
                user_dao.clone(),
                ledger.clone()
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
                rule_engine.clone()
            )),
            user_dao: user_dao.clone(),
            credit_card_dao: credit_card_dao.clone(),
            rule_engine: rule_engine.clone()
        }
    }
}