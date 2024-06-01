

#[cfg(test)]
mod tests {
    use std::fs::metadata;
    use std::sync::Arc;
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::{PaymentCancelResponse, PaymentResponse};
    use adyen_checkout::models::payment_cancel_response::Status;
    use mockall::Sequence;
    use crate::user::model::UserModel as User;
    use crate::charge::{
        service::{
            ChargeService,
            ChargeServiceTrait
        },
        constant::{
            ChargeCardAttemptResult,
            ChargeEngineResult
        }
    };
    use uuid::Uuid;
    use crate::adyen::checkout::service::*;
    use crate::asa::request::create_example_asa;
    use crate::footprint::error::FootprintError;
    use crate::passthrough_card::service::MockPassthroughCardServiceTrait;
    use crate::test_helper::{
        charge::{
            default_transaction_metadata,
            create_mock_registered_transaction,
            create_mock_failed_wallet_charge,
            create_mock_success_wallet_charge,
            create_mock_failed_passthrough_card_charge,
            create_mock_success_passthrough_card_charge
        },
        user::create_mock_user,
        passthrough_card::create_mock_passthrough_card,
        wallet::{create_mock_wallet, create_mock_wallet_with_args},
    };
    use crate::ledger::service::MockLedgerServiceTrait;
    use crate::charge::model::RegisteredTransactionModel as RegisteredTransactionModel;
    use crate::user::service::{MockUserServiceTrait, UserServiceTrait};
    use crate::footprint::service::MockFootprintServiceTrait;
    use crate::test_helper::charge::create_mock_full_transaction;
    use crate::test_helper::wallet::create_mock_wallet_with_rule;

    const USER_ID: i32 = 1;

    /*
    #[actix_web::test]
    async fn test_single_charge_fails_on_error() {
        let mut metadata = default_transaction_metadata();
        let user = create_mock_user();
        let wallet = create_mock_wallet_with_rule();
        let rtx = create_mock_registered_transaction(&metadata);

        let mut user_mock = MockUserServiceTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Err(FootprintError::Unexpected("error".into())));


        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
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
        let wallet = create_mock_wallet_with_rule();
        let rtx = create_mock_registered_transaction(&metadata);

        let user_mock = MockUserServiceTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Ok(resp));

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_success_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
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
        let wallet = create_mock_wallet_with_rule();
        let rtx = create_mock_registered_transaction(&metadata);


        let mut user_mock = MockUserServiceTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::PartiallyAuthorised);
        resp.psp_reference = Some(psp_ref.clone());

        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Ok(resp));

        let cancel_resp = PaymentCancelResponse::new(
            "SandellEnterprisesECOM".to_string(),
            psp_ref.clone(),
            "abd124".to_string(),
            Status::Received
        );
        footprint_mock.expect_proxy_adyen_cancel_request()
            .times(1)
            .return_once(move |_| Ok(cancel_resp));

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
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
        let wallet = create_mock_wallet_with_rule();
        let rtx = create_mock_registered_transaction(&metadata);

        let mut user_mock = MockUserServiceTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Refused);
        resp.psp_reference = Some(psp_ref.clone());
        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Ok(resp));

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_once( move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
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
        let mut user_mock = MockUserServiceTrait::new();
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
            .return_once(move|_| Ok(resp));

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_success_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone().into(), card_2.clone().into()],
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

        let mut user_mock = MockUserServiceTrait::new();
        let mut ledger_mock = MockLedgerServiceTrait::new();
        let mut footprint_mock = MockFootprintServiceTrait::new();

        let engine = Arc::new(ChargeService::new_with_services(
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

        let mut user_mock = MockUserServiceTrait::new();
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
            .return_once( move |_| Ok(resp_1));

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
            .return_once( move |_| Ok(resp_2));


        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_success_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone().into(), card_2.clone().into()],
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


        let mut user_mock = MockUserServiceTrait::new();
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
            .return_once(move |_| Ok(resp_1));

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
            .return_once(move |_| Ok(resp_2));

        user_mock.expect_find_by_internal_id()
            .times(0);

        ledger_mock.expect_register_successful_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_success_wallet_charge()));

        ledger_mock.expect_register_failed_inner_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        ledger_mock.expect_register_successful_outer_charge()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_success_passthrough_card_charge()));

        ledger_mock.expect_register_full_transaction()
            .times(1)
            .return_once(move |_, _, _| Ok(create_mock_full_transaction()));

        let user_txn = create_mock_registered_transaction(&metadata);
        ledger_mock.expect_register_transaction_for_user()
            .times(1)
            .return_once(move |_, _| Ok(user_txn));

        let engine = Arc::new(ChargeService::new_with_services(
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let mut asa = create_example_asa(amount_cents, mcc2.to_string());
        asa.token = Some(pc.token.to_string());
        let (res, l) = engine.clone().charge_from_asa_request(
            &asa,
            &vec![card_1.clone().into(), card_2.clone().into()],
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

        let mut user_mock = MockUserServiceTrait::new();
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
            .return_once(move |_| Ok(resp_1));

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
            .return_once( move |_| Ok(resp_2));

        let mut sequence = Sequence::new();
        ledger_mock.expect_register_failed_inner_charge()
            .once()
            .in_sequence(&mut sequence)
            .return_once(move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        ledger_mock.expect_register_failed_inner_charge()
            .once()
            .in_sequence(&mut sequence)
            .return_once(move |_, _, _| Ok(create_mock_failed_wallet_charge()));

        let engine = Arc::new(ChargeService::new_with_services(
            Arc::new(user_mock),
            Arc::new(ledger_mock),
            Arc::new(footprint_mock)
        ));
        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone().into(), card_2.clone().into()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);
    }

     */


    /*
    #[test]
    async fn test_register_transaction() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        assert_eq!(rtx.user_id, user.id);
        assert_eq!(rtx.amount_cents, metadata.amount_cents);
        assert_eq!(rtx.mcc, metadata.mcc);
        assert_eq!(rtx.memo, metadata.memo);
    }

    #[test]
    async fn test_register_failed_inner() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_failed_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Fail);
        assert_eq!(charge.is_success, None);
        assert_eq!(charge.wallet_card_id, wallet.id);
        assert_eq!(charge.user_id, user.id);
    }

    #[test]
    async fn test_register_failed_inner_rtx_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = create_mock_registered_transaction(&metadata);
        let error = ledger.clone().register_failed_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_failed_inner_card_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_mock_wallet());
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let error = ledger.clone().register_failed_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_success_inner() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Success);
        assert_eq!(charge.is_success, Some(true));
        assert_eq!(charge.wallet_card_id, wallet.id);
        assert_eq!(charge.user_id, user.id);
    }

    #[test]
    async fn test_register_success_inner_rtx_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = create_mock_registered_transaction(&metadata);
        let error = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_success_inner_card_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_mock_wallet());
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let error = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_success_inner_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Success);
        assert_eq!(charge.is_success, Some(true));
        assert_eq!(charge.wallet_card_id, wallet.id);
        assert_eq!(charge.user_id, user.id);
        let dupe = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("dupe");
        assert_eq!(LedgerError::DuplicateTransaction("test".into()), dupe);
    }

    #[test]
    async fn test_register_failed_outer() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_failed_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Fail);
        assert_eq!(charge.is_success, None);
        assert_eq!(charge.passthrough_card_id, pc.id);
        assert_eq!(charge.user_id, user.id);
    }

    #[test]
    async fn test_register_failed_outer_rtx_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = create_mock_registered_transaction(&metadata);
        let error = ledger.clone().register_failed_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect_err("should fail");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_failed_outer_card_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_mock_passthrough_card();
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let error = ledger.clone().register_failed_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect_err("should fail");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_failed_outer_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_failed_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Fail);
        assert_eq!(charge.is_success, None);
        assert_eq!(charge.passthrough_card_id, pc.id);
        assert_eq!(charge.user_id, user.id);

        let error = ledger.clone().register_failed_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect_err("should be error");
        assert_eq!(LedgerError::DuplicateTransaction("test".into()), error);
    }

    #[test]
    async fn test_register_success_outer() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Success);
        assert_eq!(charge.is_success, Some(true));
        assert_eq!(charge.passthrough_card_id, pc.id);
        assert_eq!(charge.user_id, user.id);
    }

    #[test]
    async fn test_register_success_outer_rtx_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = create_mock_registered_transaction(&metadata);
        let error = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect_err("should not register");
        assert_eq!(LedgerError::Unexpected("test".into()), error);

    }

    #[test]
    async fn test_register_success_outer_card_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_mock_passthrough_card();
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let error = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect_err("should not register");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_success_outer_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let charge = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.status, ChargeStatus::Success);
        assert_eq!(charge.is_success, Some(true));
        assert_eq!(charge.passthrough_card_id, pc.id);
        assert_eq!(charge.user_id, user.id);

        let error = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect_err("should be error");
        assert_eq!(LedgerError::DuplicateTransaction("test".into()), error);
    }

    #[test]
    async fn test_register_full() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let inner_charge = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");
        let outer_charge = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");

        let full_txn = ledger.clone().register_full_transaction(
            &rtx,
            &inner_charge,
            &outer_charge
        ).await.expect("ok");
        assert_eq!(full_txn.registered_transaction_id, rtx.id);
        assert_eq!(full_txn.outer_charge_ledger_id, outer_charge.id);
        assert_eq!(full_txn.inner_charge_ledger_id, inner_charge.id);
    }

    #[test]
    async fn test_register_full_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let inner_charge = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");
        let outer_charge = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");

        let full_txn = ledger.clone().register_full_transaction(
            &rtx,
            &inner_charge,
            &outer_charge
        ).await.expect("ok");
        assert_eq!(full_txn.registered_transaction_id, rtx.id);
        assert_eq!(full_txn.outer_charge_ledger_id, outer_charge.id);
        assert_eq!(full_txn.inner_charge_ledger_id, inner_charge.id);

        let error = ledger.clone().register_full_transaction(
            &rtx,
            &inner_charge,
            &outer_charge
        ).await.expect_err("throw dupe");
        assert_eq!(LedgerError::DuplicateTransaction("test".into()), error);
        let found = TransactionLedger::get_by_id(full_txn.id).await.expect("should still find");
        assert_eq!(found.registered_transaction_id, rtx.id);
        assert_eq!(found.outer_charge_ledger_id, outer_charge.id);
        assert_eq!(found.inner_charge_ledger_id, inner_charge.id);
    }

    #[test]
    async fn test_register_full_rtx_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let inner_charge = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");
        let outer_charge = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");

        let error = ledger.clone().register_full_transaction(
            &create_mock_registered_transaction(&metadata),
            &inner_charge,
            &outer_charge
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_full_inner_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");

        let outer_charge = ledger.clone().register_successful_outer_charge(
            &rtx,
            &metadata,
            &pc
        ).await.expect("ok");

        let error = ledger.clone().register_full_transaction(
            &rtx,
            &create_mock_success_inner_charge(),
            &outer_charge
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_register_full_outer_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = WalletModelWithRule::from(create_wallet(&user).await);
        let pc = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await.expect("ok");
        let inner_charge = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect("ok");

        let error = ledger.clone().register_full_transaction(
            &create_mock_registered_transaction(&metadata),
            &inner_charge,
            &create_mock_success_outer_charge()
        ).await.expect_err("ok");
        assert_eq!(LedgerError::Unexpected("test".into()), error);
    }
     */
}