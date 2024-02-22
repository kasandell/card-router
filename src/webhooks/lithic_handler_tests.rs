#[cfg(test)]
mod tests {
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::PaymentResponse;
    use crate::wallet::entity::Wallet;
    use crate::adyen_service::checkout::service::MockAdyenChargeServiceTrait;
    use crate::asa::request::create_example_asa;
    use crate::asa::response::AsaResponseResult;
    use crate::rule_engine::engine::MockRuleEngineTrait;
    use crate::user::entity::User;
    use crate::webhooks::lithic_handler::LithicHandler;
    use std::default::Default;
    use crate::schema::passthrough_card::dsl::passthrough_card;
    use crate::schema::wallet::payment_method_id;
    use crate::test_helper::{default_transaction_metadata, initialize_passthrough_card, initialize_registered_transaction_for_user, initialize_user, initialize_wallet, initialize_wallet_with_payment_method};
    use crate::transaction::entity::{InnerChargeLedger, RegisteredTransaction};

    #[actix_web::test]
    pub async fn test_handle() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        metadata.amount_cents = 100;
        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let pc = initialize_passthrough_card(
            &user,
        );
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let (card_1, ca1) = initialize_wallet_with_payment_method(&user, 1, payment_method_1.to_string());
        let (card_2, ca2) = initialize_wallet_with_payment_method(&user, 2, payment_method_2.to_string());
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut rule_engine = MockRuleEngineTrait::new();




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

        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc1
                        && charge_request.payment_method_id == payment_method_1.to_string()
                        && charge_request.customer_public_id == user.public_id
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp_1
                )
            );

        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc2
                        && charge_request.payment_method_id == payment_method_2.to_string()
                        && charge_request.customer_public_id == user.public_id
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp_2
                )
            );

        let handler = LithicHandler::new_with_engines(
            Box::new(charge_service),
            Box::new(rule_engine)
        );

        let mut asa = create_example_asa(
            amount_cents,
            mcc.to_string()
        );
        asa.token = pc.token.clone();


        let res = handler.handle(
            asa.clone()
        ).await.expect("no error");
        assert_eq!(AsaResponseResult::Approved, res.result);
        assert_eq!(asa.token, res.token);
        InnerChargeLedger::delete_all();
        RegisteredTransaction::delete_all();
        card_1.delete_self().expect("should delete");
        ca1.delete_self().expect("should delete");
        card_2.delete_self().expect("should delete");
        ca2.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        pc.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

}