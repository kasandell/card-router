#[cfg(test)]
mod tests {
    use adyen_webhooks::models::{Amount, RecurringContractNotificationRequestItem, RecurringContractNotificationRequestItemWrapper};
    use adyen_webhooks::models::recurring_contract_notification_request_item::EventCode;
    use crate::wallet::entity::{InsertableCardAttempt, Wallet, WalletCardAttempt};
    use crate::test_helper::initialize_user;
    use crate::wallet::constant::WalletCardAttemptStatus;
    use crate::webhooks::adyen_handler::AdyenHandler;
    use crate::charge_engine::engine::Engine as ChargeEngine;
    use crate::adyen_service::checkout::service::MockAdyenChargeServiceTrait;
    use crate::rule_engine::engine::{MockRuleEngineTrait, RuleEngineTrait};
    use crate::webhooks::lithic_handler::LithicHandler;

    #[actix_web::test]
    pub async fn test_handle() {
        let charge_service = MockAdyenChargeServiceTrait::new();
        let rule_engine = MockRuleEngineTrait::new();
        let handler = LithicHandler::new_with_engines(
            Box::new(charge_service),
            Box::new(rule_engine)
        );



    }

}