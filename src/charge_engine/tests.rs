

#[cfg(test)]
mod tests {
    use std::fs::metadata;
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::{PaymentCancelResponse, PaymentResponse};
    use adyen_checkout::models::payment_cancel_response::Status;
    use chrono::NaiveDateTime;
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
    use crate::passthrough_card::dao::MockPassthroughCardDaoTrait;
    use crate::passthrough_card::entity::PassthroughCard;
    use crate::test_helper::{default_transaction_metadata, initialize_registered_transaction_for_user, initialize_user, initialize_wallet, initialize_wallet_with_payment_method, initialize_passthrough_card, create_failed_inner_charge, create_success_outer_charge, create_success_inner_charge, create_full_transaction, create_passthrough_card, create_registered_transaction};
    use crate::transaction::constant::ChargeStatus;
    use crate::transaction::engine::MockTransactionEngineTrait;
    use crate::transaction::entity::{InnerChargeLedger, OuterChargeLedger, RegisteredTransaction, TransactionMetadata};
    use crate::user::dao::{MockUserDaoTrait, UserDaoTrait};
    use chrono::Utc;
    use crate::schema::wallet::payment_method_id;

    const USER_ID: i32 = 1;

    #[actix_web::test]
    async fn test_single_charge_fails_on_error() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(USER_ID);
        let wallet = Wallet::create_test_wallet( 1, USER_ID, 1 );
        let rtx = RegisteredTransaction::create_test_transaction( 1, &metadata );

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();
        charge_service.expect_charge_card_on_file()
            .times(1)
            .return_const(
                Err(ServiceError::new(500, "test_error".to_string()))
            );

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_failed_inner_charge(USER_ID)
                )
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Denied, res);
    }

    #[actix_web::test]
    async fn test_single_charge_succeeds() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(USER_ID);
        let wallet = Wallet::create_test_wallet( 1, USER_ID, 1 );
        let rtx = RegisteredTransaction::create_test_transaction( USER_ID, &metadata );

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        charge_service.expect_charge_card_on_file()
            .times(1)
            .return_const(
                Ok(
                    resp
                )
            );

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_success_inner_charge(USER_ID)
                )
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Approved, res);
    }

    #[actix_web::test]
    async fn test_single_charge_needs_cancel_and_succeeds() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(USER_ID);
        let wallet = Wallet::create_test_wallet( 1, USER_ID, 1 );
        let rtx = RegisteredTransaction::create_test_transaction( USER_ID, &metadata );


        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();
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

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(create_failed_inner_charge(USER_ID))
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::PartialCancelSucceeded, res);
    }

    #[actix_web::test]
    async fn test_single_charge_does_not_go_through() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(USER_ID);
        let wallet = Wallet::create_test_wallet( 1, USER_ID, 1 );
        let rtx = RegisteredTransaction::create_test_transaction( USER_ID, &metadata );

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();
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

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_failed_inner_charge(USER_ID)
                )
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Denied, res);

    }


    #[actix_web::test]
    async fn test_charge_user_wallet_first_card_success() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(1);
        let mut rtx = RegisteredTransaction::create_test_transaction( 1, &metadata );

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let mc_clone = metadata.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let mut card_1 = Wallet::create_test_wallet( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = Wallet::create_test_wallet( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();



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

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_success_inner_charge(USER_ID)
                )
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Approved, res);
    }

    #[actix_web::test]
    async fn test_charge_user_wallet_no_cards_fails() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(1);
        let mut rtx = RegisteredTransaction::create_test_transaction( 1, &metadata );

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(1);
        let mut rtx = RegisteredTransaction::create_test_transaction( 1, &metadata );

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let mc_clone = metadata.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = Wallet::create_test_wallet( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = Wallet::create_test_wallet( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();

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

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_failed_inner_charge(USER_ID)
                )
            );

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_success_inner_charge(USER_ID)
                )
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Approved, res);
    }

    #[actix_web::test]
    async fn test_charge_user_wallet_second_card_from_asa() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(1);
        let mut rtx = RegisteredTransaction::create_test_transaction( 1, &metadata );

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = Wallet::create_test_wallet( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = Wallet::create_test_wallet( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();

        let pc = create_passthrough_card(&user);


        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();

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
                        && charge_request.mcc == mcc1
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
                Ok(create_success_inner_charge(USER_ID))
            );
        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(create_failed_inner_charge(USER_ID))
            );
        ledger_mock.expect_register_successful_outer_charge()
            .times(1)
            .return_const(
                Ok(create_success_outer_charge(USER_ID))
            );
        ledger_mock.expect_register_full_transaction()
            .times(1)
            .return_const(
                Ok(create_full_transaction())
            );
        ledger_mock.expect_register_transaction_for_user()
            .times(1)
            .return_const(
                Ok(create_registered_transaction())
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let mut asa = create_example_asa(amount_cents, mcc2.to_string());
        asa.token = pc.token.to_string();
        let (res, l) = engine.charge_from_asa_request(
            &asa,
            &vec![card_1.clone(), card_2.clone()]
        ).await.expect("no error");
        assert_eq!(res, ChargeEngineResult::Approved);
        let ledger = l.expect("Should exist");
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card_fails() {
        let mut metadata = default_transaction_metadata();
        let user = User::create_test_user(1);
        let mut rtx = RegisteredTransaction::create_test_transaction( 1, &metadata );

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = Wallet::create_test_wallet( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = Wallet::create_test_wallet( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockTransactionEngineTrait::new();

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

        ledger_mock.expect_register_failed_inner_charge()
            .times(2)
            .return_const(
                Ok(create_failed_inner_charge(USER_ID))
            );

        let engine = Engine::new_with_service(
            Box::new(charge_service),
            Box::new(pc_mock),
            Box::new(user_mock),
            Box::new(ledger_mock)
        );
        let (res, ledger) = engine.charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);
    }
}