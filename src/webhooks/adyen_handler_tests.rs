#[cfg(test)]
mod tests {

    use std::sync::Arc;
    use adyen_webhooks::models::{Amount, RecurringContractNotificationRequestItem, RecurringContractNotificationRequestItemWrapper};
    use adyen_webhooks::models::recurring_contract_notification_request_item::EventCode;
    use crate::wallet::entity::{InsertableCardAttempt, Wallet, WalletCardAttempt};
    use crate::test_helper::initialize_user;
    use crate::wallet::constant::WalletCardAttemptStatus;
    use crate::webhooks::adyen_handler::AdyenHandler;

    #[actix_web::test]
    async fn test_successful_match() {
        let user = initialize_user().await;
        let attempt_reference_id = "abcd";
        let original_psp = "xywz";
        let payment_method_id = "1234";
        let att1 = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 1,
                expected_reference_id: attempt_reference_id
            }
        ).await.expect("should create");
        assert_eq!(att1.status, WalletCardAttemptStatus::PENDING.as_str());
        let adyen_handler = Arc::new(AdyenHandler::new());
        let _ = adyen_handler.clone().handle_item(
            RecurringContractNotificationRequestItemWrapper {
                notification_request_item: Some(
                    Box::new(
                        RecurringContractNotificationRequestItem {
                            additional_data: None,
                            amount: Box::new(
                                Amount {
                                    currency: "USD".to_string(),
                                    value: 0
                                }
                            ),
                            event_code: EventCode::RecurringContract,
                            event_date: "2023-12-01".to_string(),
                            original_psp: original_psp.to_string(),
                            merchant_account_code: "".to_string(),
                            merchant_reference: attempt_reference_id.to_string(),
                            psp_reference: payment_method_id.to_string(),
                            original_reference: None,
                            payment_method: None,
                            reason: None,
                            success: "true".to_string(),
                        }
                    )
                )
            }
        ).await.expect("should be fine");
        let att_returned = WalletCardAttempt::find_by_reference_id(attempt_reference_id).await.expect("should create");
        assert_eq!(att_returned.id, att1.id);
        assert_eq!(att_returned.status, WalletCardAttemptStatus::MATCHED.as_str());
        let wallet_returned = Wallet::find_all_for_user(&user).await.expect("should get wallet");
        assert_eq!(wallet_returned.len(), 1);
        let card = &wallet_returned[0];
        assert_eq!(card.wallet_card_attempt_id, att_returned.id);
        assert_eq!(card.payment_method_id, payment_method_id.to_string());
        assert_eq!(card.user_id, user.id);
        assert_eq!(card.credit_card_id, 1);
        card.delete_self().await.expect("should delete");
        att_returned.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete user");

    }
}