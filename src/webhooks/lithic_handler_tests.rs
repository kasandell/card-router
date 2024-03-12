#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::PaymentResponse;
    use crate::wallet::entity::Wallet;
    use crate::adyen_service::checkout::service::MockAdyenChargeServiceTrait;
    use crate::asa::request::create_example_asa;
    use crate::asa::response::AsaResponseResult;
    use crate::rule_engine::engine::MockRuleEngineTrait;
    use crate::webhooks::lithic_handler::LithicHandler;
    use std::default::Default;
    use crate::footprint_service::service::MockFootprintServiceTrait;
    use crate::passthrough_card::dao::MockPassthroughCardDaoTrait;
    use crate::test_helper::ledger::{create_mock_failed_inner_charge, create_mock_full_transaction, create_mock_registered_transaction, create_mock_success_inner_charge, create_mock_success_outer_charge, default_transaction_metadata};
    use crate::test_helper::passthrough_card::create_mock_passthrough_card;
    use crate::test_helper::user::create_mock_user;
    use crate::transaction::engine::MockTransactionEngineTrait;
    use crate::transaction::entity::{InnerChargeLedger, RegisteredTransaction};
    use crate::user::dao::MockUserDaoTrait;

    // TODO: how to use the mocks appropriately here / how to share them
    #[actix_web::test]
    pub async fn test_handle() {
        let mut metadata = default_transaction_metadata();
        let user = create_mock_user();
        let mut rtx = RegisteredTransaction::create_test_transaction( 1, &metadata ).await;

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

        let passthrough_card_token = "pc_token_123";
        let mut pc = create_mock_passthrough_card();
        pc.token = passthrough_card_token.to_string();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = Wallet::create_test_wallet( 1, 1, 1 ).await;
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = Wallet::create_test_wallet( 2, 1, 2 ).await;
        card_2.payment_method_id = payment_method_2.to_string();

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();
        let mut rule_engine = MockRuleEngineTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Authorised);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        let cards = vec![card_1.clone(), card_2.clone()];

        rule_engine.expect_order_user_cards_for_request()
            .times(1)
            .return_const(
                Ok(
                    cards
                )
            );

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc1
                        && charge_request.payment_method_id == payment_method_1.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp_1
                )
            );

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc2
                        && charge_request.payment_method_id == payment_method_2.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp_2
                )
            );

        pc_mock.expect_get_by_token()
            .times(1)
            .return_const(
                Ok(pc.clone())
            );

        user_mock.expect_find_by_internal_id()
            .times(1)
            .return_const(
                Ok(user)
            );

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_const(
                Ok(create_mock_success_inner_charge())
            );
        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(create_mock_failed_inner_charge())
            );
        ledger_mock.expect_register_successful_outer_charge()
            .times(1)
            .return_const(
                Ok(create_mock_success_outer_charge())
            );
        ledger_mock.expect_register_full_transaction()
            .times(1)
            .return_const(
                Ok(create_mock_full_transaction())
            );
        ledger_mock.expect_register_transaction_for_user()
            .times(1)
            .return_const(
                Ok(create_mock_registered_transaction(&metadata))
            );

        let handler = Arc::new(LithicHandler::new_with_engines(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(rule_engine),
            Arc::new(footprint_mock)
        ));

        let mut asa = create_example_asa(
            amount_cents,
            mcc.to_string()
        );
        asa.token = Some("test_token_123".to_string());
        asa.card.as_mut().unwrap().token = Some(pc.token.clone());


        let res = handler.clone().handle(
            asa.clone()
        ).await.expect("no error");
        assert_eq!(AsaResponseResult::Approved, res.result);
        // TODO: is this token correct
        assert_eq!(passthrough_card_token, res.token);
    }


}