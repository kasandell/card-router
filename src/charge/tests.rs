

#[cfg(test)]
mod tests {
    use std::fs::metadata;
    use std::sync::Arc;
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::{PaymentCancelResponse, PaymentResponse};
    use adyen_checkout::models::payment_cancel_response::Status;
    use crate::user::entity::User;
    use crate::error::service_error::ServiceError;
    use crate::charge::{
        service::ChargeService,
        entity::{
            ChargeCardAttemptResult,
            ChargeEngineResult
        }
    };
    use uuid::Uuid;
    use crate::adyen::checkout::service::*;
    use crate::asa::request::create_example_asa;
    use crate::passthrough_card::dao::MockPassthroughCardDaoTrait;
    use crate::test_helper::{
        ledger::{
            default_transaction_metadata,
            create_mock_registered_transaction,
            create_mock_failed_inner_charge,
            create_mock_success_inner_charge,
            create_mock_failed_outer_charge,
            create_mock_success_outer_charge
        },
        user::create_mock_user,
        passthrough_card::create_mock_passthrough_card,
        wallet::{create_mock_wallet, create_mock_wallet_with_args},
    };
    use crate::ledger::service::MockLedgerServiceTrait;
    use crate::ledger::entity::RegisteredTransaction;
    use crate::user::dao::{MockUserDaoTrait, UserDaoTrait};
    use crate::footprint::service::MockFootprintServiceTrait;
    use crate::test_helper::ledger::create_mock_full_transaction;

    const USER_ID: i32 = 1;

    #[actix_web::test]
    async fn test_single_charge_fails_on_error() {
        let mut metadata = default_transaction_metadata();
        let user = create_mock_user();
        let wallet = create_mock_wallet();
        let rtx = create_mock_registered_transaction(&metadata);

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_const(
                Err(ServiceError::new(ErrorType::InternalServerError, "test_error"))
            );

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_mock_failed_inner_charge()
                )
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_card_with_cleanup(
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
        let metadata = default_transaction_metadata();
        let user = create_mock_user();
        let wallet = create_mock_wallet();
        let rtx = create_mock_registered_transaction(&metadata);

        let charge_service = MockAdyenChargeServiceTrait::new();
        let user_mock = MockUserDaoTrait::new();
        let pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        footprint_mock.expect_proxy_adyen_payment_request()
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
                    create_mock_success_inner_charge()
                )
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_card_with_cleanup(
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
        let user = create_mock_user();
        let wallet = create_mock_wallet();
        let rtx = create_mock_registered_transaction(&metadata);


        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::PartiallyAuthorised);
        resp.psp_reference = Some(psp_ref.clone());

        footprint_mock.expect_proxy_adyen_payment_request()
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
                Ok(create_mock_failed_inner_charge())
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_card_with_cleanup(
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
        let user = create_mock_user();
        let wallet = create_mock_wallet();
        let rtx = create_mock_registered_transaction(&metadata);

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Refused);
        resp.psp_reference = Some(psp_ref.clone());
        footprint_mock.expect_proxy_adyen_payment_request()
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
                    create_mock_failed_inner_charge()
                )
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_card_with_cleanup(
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
        let user = create_mock_user();
        let mut rtx = create_mock_registered_transaction(&metadata);
        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let mc_clone = metadata.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";
        let mut card_1 = create_mock_wallet_with_args( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = create_mock_wallet_with_args( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();



        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        resp.psp_reference = Some(psp_ref.clone());
        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == mc_clone.amount_cents
                        && charge_request.mcc == mc_clone.mcc
                        && charge_request.payment_method_id == payment_method_1.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
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
                    create_mock_success_inner_charge()
                )
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
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
        let user = create_mock_user();
        let mut rtx = create_mock_registered_transaction(&metadata);

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
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
        let user = create_mock_user();
        let mut rtx = create_mock_registered_transaction(&metadata);

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let mc_clone = metadata.clone();
        let mc_clone2 = metadata.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = create_mock_wallet_with_args( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = create_mock_wallet_with_args( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Authorised);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == mc_clone.amount_cents
                        && charge_request.mcc == mc_clone.mcc
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
                    charge_request.amount_cents == mc_clone2.amount_cents
                        && charge_request.mcc == mc_clone2.mcc
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


        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_mock_failed_inner_charge()
                )
            );

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_const(
                Ok(
                    create_mock_success_inner_charge()
                )
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
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
        let user = create_mock_user();
        let mut rtx = create_mock_registered_transaction(&metadata);

        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let mcc3 = metadata.mcc.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = create_mock_wallet_with_args( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = create_mock_wallet_with_args( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();

        let pc = create_mock_passthrough_card();


        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Authorised);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == 100
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
                    charge_request.amount_cents == 100
                        && charge_request.mcc == mcc3
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
            .times(0);

        user_mock.expect_find_by_internal_id()
            .times(0);

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

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let mut asa = create_example_asa(amount_cents, mcc2.to_string());
        asa.token = Some(pc.token.to_string());
        let (res, l) = engine.clone().charge_from_asa_request(
            &asa,
            &vec![card_1.clone(), card_2.clone()],
            &pc,
            &user
        ).await.expect("no error");
        assert_eq!(res, ChargeEngineResult::Approved);
        let ledger = l.expect("Should exist");
    }


    #[actix_web::test]
    async fn test_charge_user_wallet_second_card_fails() {
        let mut metadata = default_transaction_metadata();
        let user = create_mock_user();
        let mut rtx = create_mock_registered_transaction(&metadata);
        metadata.amount_cents = 100;
        rtx.amount_cents = 100;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut card_1 = create_mock_wallet_with_args( 1, 1, 1 );
        card_1.payment_method_id = payment_method_1.to_string();
        let mut card_2 = create_mock_wallet_with_args( 2, 1, 2 );
        card_2.payment_method_id = payment_method_2.to_string();

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut user_mock = MockUserDaoTrait::new();
        let mut pc_mock = MockPassthroughCardDaoTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Refused);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc
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

        ledger_mock.expect_register_failed_inner_charge()
            .times(2)
            .return_const(
                Ok(create_mock_failed_inner_charge())
            );

        let engine = Arc::new(ChargeService::new_with_service(
            Arc::new(charge_service),
            Arc::new(pc_mock),
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone(), card_2.clone()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);
    }
}