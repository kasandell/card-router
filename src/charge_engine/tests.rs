

#[cfg(test)]
mod tests {
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::{PaymentCancelResponse, PaymentResponse};
    use adyen_checkout::models::payment_cancel_response::Status;
    use crate::wallet::entity::Wallet;
    use crate::user::entity::User;
    use crate::adyen_service::checkout::error::Error;
    use crate::charge_engine::engine::Engine;
    use uuid::Uuid;
    use crate::adyen_service::checkout::service::*;

    #[actix_web::test]
    async fn test_single_charge_fails_on_error() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        charge_service.expect_charge_card_on_file()
            .return_const(
                Err(Error::new("test_error".to_string()))
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &Wallet::create_test_wallet(
                1,
                1,
                1
            ),
            &User::create_test_user(
                1
            ),
            0,
            "7184",
            "Test charge"
        ).await.expect("NO error");
        assert!(!res);
    }

    #[actix_web::test]
    async fn test_single_charge_succeeds() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        charge_service.expect_charge_card_on_file()
            .return_const(
                Ok(
                    resp
                )
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &Wallet::create_test_wallet(
                1,
                1,
                1
            ),
            &User::create_test_user(
                1
            ),
            0,
            "7184",
            "Test charge"
        ).await.expect("NO error");
        assert!(res);
    }

    #[actix_web::test]
    async fn test_single_charge_needs_cancel_and_succeeds() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::PartiallyAuthorised);
        resp.psp_reference = Some(psp_ref.clone());
        charge_service.expect_charge_card_on_file()
            .times(1)
            .return_const(
                Ok(
                    resp
                )
            );

        let cancel_resp = PaymentCancelResponse::new(
            "SandellEnterprisesECOM".to_string(),
            psp_ref.clone(),
            "abd124".to_string(),
            Status::Received
        );
        charge_service.expect_cancel_transaction()
            .times(1)
            .return_const(
                Ok(cancel_resp)
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &Wallet::create_test_wallet(
                1,
                1,
                1
            ),
            &User::create_test_user(
                1
            ),
            0,
            "7184",
            "Test charge"
        ).await.expect("NO error");
        assert!(!res);
    }

    #[actix_web::test]
    async fn test_single_charge_does_not_go_through() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Refused);
        resp.psp_reference = Some(psp_ref.clone());
        charge_service.expect_charge_card_on_file()
            .times(1)
            .return_const(
                Ok(
                    resp
                )
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &Wallet::create_test_wallet(
                1,
                1,
                1
            ),
            &User::create_test_user(
                1
            ),
            0,
            "7184",
            "Test charge"
        ).await.expect("NO error");
        assert!(!res);
    }

    #[actix_web::test]
    async fn test_charge_user_wallet_first_card_success() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let user_id = 1;
        let amount_cents = 100;
        let mcc = "7184";
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let user = User::create_test_user(
            user_id
        );
        let mut card_1 = Wallet::create_test_wallet(
            1,
            user_id,
            1,
        );
        card_1.payment_method_id = payment_method_1.to_string();

        let mut card_2 = Wallet::create_test_wallet(
            2,
            user_id,
            2
        );
        card_2.payment_method_id = payment_method_2.to_string();


        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        resp.psp_reference = Some(psp_ref.clone());
        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                    && charge_request.mcc == mcc
                    && charge_request.payment_method_id == payment_method_1.to_string()
                    && charge_request.customer_public_id == user.public_id
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp
                )
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_wallet(
            &user,
            &vec![card_1, card_2],
            amount_cents,
            mcc,
            "test statement"
        ).await.expect("NO error");
        assert!(res);
    }

    #[actix_web::test]
    async fn test_charge_user_wallet_no_cards_fails() {
        let charge_service = MockAdyenChargeServiceTrait::new();
        let user_id = 1;
        let amount_cents = 100;
        let mcc = "7184";

        let user = User::create_test_user(
            user_id
        );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_wallet(
            &user,
            &vec![],
            amount_cents,
            mcc,
            "test statement"
        ).await.expect("NO error");
        assert!(!res);
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let user_id = 1;
        let amount_cents = 100;
        let mcc = "7184";
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let user = User::create_test_user(
            user_id
        );
        let mut card_1 = Wallet::create_test_wallet(
            1,
            user_id,
            1,
        );
        card_1.payment_method_id = payment_method_1.to_string();

        let mut card_2 = Wallet::create_test_wallet(
            2,
            user_id,
            2
        );
        card_2.payment_method_id = payment_method_2.to_string();


        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Authorised);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc
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
                        && charge_request.mcc == mcc
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

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_wallet(
            &user,
            &vec![card_1, card_2],
            amount_cents,
            mcc,
            "test statement"
        ).await.expect("NO error");
        assert!(res);
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card_fails() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let user_id = 1;
        let amount_cents = 100;
        let mcc = "7184";
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let user = User::create_test_user(
            user_id
        );
        let mut card_1 = Wallet::create_test_wallet(
            1,
            user_id,
            1,
        );
        card_1.payment_method_id = payment_method_1.to_string();

        let mut card_2 = Wallet::create_test_wallet(
            2,
            user_id,
            2
        );
        card_2.payment_method_id = payment_method_2.to_string();


        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Refused);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc
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
                        && charge_request.mcc == mcc
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

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_wallet(
            &user,
            &vec![card_1, card_2],
            amount_cents,
            mcc,
            "test statement"
        ).await.expect("NO error");
        assert!(!res);
    }
}