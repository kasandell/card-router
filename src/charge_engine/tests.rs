

#[cfg(test)]
mod tests {
    use std::fs::metadata;
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::{PaymentCancelResponse, PaymentResponse};
    use adyen_checkout::models::payment_cancel_response::Status;
    use crate::wallet::entity::Wallet;
    use crate::user::entity::User;
    use crate::service_error::ServiceError;
    use crate::charge_engine::{
        engine::Engine,
        entity::{
            ChargeCardAttemptResult,
            ChargeEngineResult
        }
    };
    use uuid::Uuid;
    use crate::adyen_service::checkout::service::*;
    use crate::asa::request::create_example_asa;
    use crate::test_helper::{default_transaction_metadata, initialize_registered_transaction_for_user, initialize_user, initialize_wallet, initialize_wallet_with_payment_method, initialize_passthrough_card};
    use crate::transaction::constant::ChargeStatus;
    use crate::transaction::entity::{InnerChargeLedger, OuterChargeLedger, RegisteredTransaction, TransactionMetadata};

    #[actix_web::test]
    async fn test_single_charge_fails_on_error() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let (wallet, ca) = initialize_wallet(&user, 1);
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        charge_service.expect_charge_card_on_file()
            .return_const(
                Err(ServiceError::new(500, "test_error".to_string()))
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Denied, res);
        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        wallet.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }

    #[actix_web::test]
    async fn test_single_charge_succeeds() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let (wallet, ca) = initialize_wallet(&user, 1);
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
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Approved, res);

        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");

        wallet.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }

    #[actix_web::test]
    async fn test_single_charge_needs_cancel_and_succeeds() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let (wallet, ca) = initialize_wallet(&user, 1);
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
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::PartialCancelSucceeded, res);
        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        wallet.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }

    #[actix_web::test]
    async fn test_single_charge_does_not_go_through() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let (wallet, ca) = initialize_wallet(&user, 1);
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
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Denied, res);

        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        wallet.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_first_card_success() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        metadata.amount_cents = 100;
        let mc_clone = metadata.clone();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let (card_1, ca1) = initialize_wallet_with_payment_method(&user, 1, payment_method_1.to_string());
        let (card_2, ca2) = initialize_wallet_with_payment_method(&user, 2, payment_method_2.to_string());
        let mut charge_service = MockAdyenChargeServiceTrait::new();



        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        resp.psp_reference = Some(psp_ref.clone());
        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == mc_clone.amount_cents
                    && charge_request.mcc == mc_clone.mcc
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
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Approved, res);

        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        card_1.delete_self().expect("should delete");
        ca1.delete_self().expect("should delete");
        card_2.delete_self().expect("should delete");
        ca2.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }

    #[actix_web::test]
    async fn test_charge_user_wallet_no_cards_fails() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        metadata.amount_cents = 100;
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let charge_service = MockAdyenChargeServiceTrait::new();
        let user_id = 1;
        let amount_cents = 100;
        let mcc = "7184";

        let user = User::create_test_user(
            user_id
        );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);

        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card() {
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        metadata.amount_cents = 100;
        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let mc_clone = metadata.clone();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let (card_1, ca1) = initialize_wallet_with_payment_method(&user, 1, payment_method_1.to_string());
        let (card_2, ca2) = initialize_wallet_with_payment_method(&user, 2, payment_method_2.to_string());
        let mut charge_service = MockAdyenChargeServiceTrait::new();

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

        let engine = Engine::new_with_service(Box::new(charge_service));
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Approved, res);

        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        card_1.delete_self().expect("should delete");
        ca1.delete_self().expect("should delete");
        card_2.delete_self().expect("should delete");
        ca2.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }

    #[actix_web::test]
    async fn test_charge_user_wallet_second_card_from_asa() {
        crate::test::init();
        let user = initialize_user();
        let amount_cents = 100;
        let mcc = "7184";
        let pc = initialize_passthrough_card(
            &user
        );

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let (card_1, ca1) = initialize_wallet_with_payment_method(&user, 1, payment_method_1.to_string());
        let (card_2, ca2) = initialize_wallet_with_payment_method(&user, 2, payment_method_2.to_string());
        let mut charge_service = MockAdyenChargeServiceTrait::new();

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
        let mut asa = create_example_asa(amount_cents, mcc.to_string());
        asa.token = pc.token.to_string();
        let (res, l) = engine.charge_from_asa_request(
            &asa,
            &vec![card_1.clone(), card_2.clone()]
        ).await.expect("no error");
        assert_eq!(res, ChargeEngineResult::Approved);
        let ledger = l.expect("Should exist");
        let rtx = RegisteredTransaction::get_by_transaction_id(
            ledger.transaction_id.clone()
        ).expect("should exist");
        assert_eq!(rtx.amount_cents, amount_cents);
        assert_eq!(rtx.mcc, mcc.to_string());
        assert_eq!(rtx.user_id, user.id);
        assert_eq!(rtx.memo, asa.merchant.descriptor);

        let inner_charge = InnerChargeLedger::get_by_id(ledger.inner_charge_ledger_id).expect("should exist");
        assert_eq!(inner_charge.amount_cents, amount_cents);
        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.wallet_card_id, card_2.id);
        assert_eq!(inner_charge.status, ChargeStatus::Success.as_str());

        let outer_charge = OuterChargeLedger::get_by_id(ledger.outer_charge_ledger_id).expect("should exist");
        assert_eq!(outer_charge.amount_cents, amount_cents);
        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.passthrough_card_id, pc.id);
        assert_eq!(outer_charge.status, ChargeStatus::Success.as_str());


        ledger.delete_self().expect("should delete");
        InnerChargeLedger::delete_all().expect("should delete");
        OuterChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        pc.delete_self().expect("should delete");
        card_1.delete_self().expect("should delete");
        ca1.delete_self().expect("should delete");
        card_2.delete_self().expect("should delete");
        ca2.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card_fails() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        crate::test::init();
        let user = initialize_user();
        let mut metadata = default_transaction_metadata();
        metadata.amount_cents = 100;
        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let rtx = initialize_registered_transaction_for_user(
            &user,
            &metadata
        );
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let (card_1, ca1) = initialize_wallet_with_payment_method(&user, 1, payment_method_1.to_string());
        let (card_2, ca2) = initialize_wallet_with_payment_method(&user, 2, payment_method_2.to_string());


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

        let engine = Engine::new_with_service(Box::new(charge_service));
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);

        InnerChargeLedger::delete_all().expect("should delete");
        RegisteredTransaction::delete_all().expect("should delete");
        card_1.delete_self().expect("should delete");
        ca1.delete_self().expect("should delete");
        card_2.delete_self().expect("should delete");
        ca2.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
        User::delete_all().expect("should delete");
    }
}