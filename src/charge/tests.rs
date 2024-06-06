

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::{PaymentCancelResponse, PaymentResponse};
    use adyen_checkout::models::payment_cancel_response::Status;
    use crate::user::model::{UserModel as User, UserModel};
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
    use crate::asa::request::create_example_asa;
    use crate::footprint::error::FootprintError;
    use crate::test_helper::{
        charge::{
            default_transaction_metadata,
        },
    };
    use crate::charge::model::RegisteredTransactionModel as RegisteredTransactionModel;
    use crate::user::service::{UserService, UserServiceTrait};
    use crate::footprint::service::MockFootprintServiceTrait;
    use crate::test_helper::user::create_user;
    use actix_web::test;
    use crate::asa::response::AsaResponseResult;
    use crate::charge::dao::{ChargeDao, ChargeDaoTrait};
    use crate::charge::entity::InsertableRegisteredTransaction;
    use crate::common::model::TransactionMetadata;
    use crate::error::data_error::DataError;
    use crate::ledger::service::LedgerService;
    use crate::test_helper::passthrough_card::create_passthrough_card;
    use crate::test_helper::wallet::{create_wallet, create_wallet_with_rule};
    use crate::util::transaction::transactional;

    const USER_ID: i32 = 1;

    #[test]
    async fn test_single_charge_fails_on_error() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;

        let mut footprint_mock = MockFootprintServiceTrait::new();
        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Err(FootprintError::Unexpected("error".into())));

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));

        let rtx = create_registered_transaction(&user, &metadata).await;

        let (res, ledger) = engine.clone().charge_card_with_cleanup(
            Uuid::new_v4(),
            &wallet,
            &user,
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeCardAttemptResult::Denied, res);
    }

    #[test]
    async fn test_single_charge_succeeds() {
        crate::test_helper::general::init();
        let metadata = default_transaction_metadata();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;
        let rtx = create_registered_transaction(&user, &metadata).await;
        let mut footprint_mock = MockFootprintServiceTrait::new();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Ok(resp));

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());



        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service.clone(),
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

    #[test]
    #[ignore]
    // TODO: turned off cancellation logic for now
    async fn test_single_charge_needs_cancel_and_succeeds() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;
        let rtx = create_registered_transaction(&user, &metadata).await;


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

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
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

    #[test]
    async fn test_single_charge_does_not_go_through() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;
        let rtx = create_registered_transaction(&user, &metadata).await;

        let mut footprint_mock = MockFootprintServiceTrait::new();
        let psp_ref = "abc123".to_string();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Refused);
        resp.psp_reference = Some(psp_ref.clone());
        footprint_mock.expect_proxy_adyen_payment_request()
            .times(1)
            .return_once(move |_| Ok(resp));

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
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


    #[test]
    async fn test_charge_user_wallet_first_card_success() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let mut rtx = create_registered_transaction(&user, &metadata).await;
        let mc_clone = metadata.clone();
        let card_1 = create_wallet_with_rule(&user).await;
        let payment_method_1 = card_1.payment_method_id.clone();
        let card_2 = create_wallet_with_rule(&user).await;
        let payment_method_2 = card_2.payment_method_id.clone();

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

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));

        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone().into(), card_2.clone().into()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Approved, res);
    }

    #[test]
    async fn test_charge_user_wallet_no_cards_fails() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let mut rtx = create_registered_transaction(&user, &metadata).await;

        let mut footprint_mock = MockFootprintServiceTrait::new();
        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));
        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);
    }


    #[test]
    async fn test_charge_user_wallet_second_card() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let mut rtx = create_registered_transaction(&user, &metadata).await;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();
        let mc_clone = metadata.clone();
        let mc_clone2 = metadata.clone();


        let mut card_1 = create_wallet_with_rule(&user).await;
        let mut card_2 = create_wallet_with_rule(&user).await;

        let payment_method_1 = card_1.payment_method_id.clone();
        let payment_method_2 = card_2.payment_method_id.clone();

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

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));

        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone().into(), card_2.clone().into()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Approved, res);
    }

    #[test]
    async fn test_charge_user_wallet_second_card_from_asa() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let mut rtx = create_registered_transaction(&user, &metadata).await;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();

        let mut card_1 = create_wallet(&user).await;
        let mut card_2 = create_wallet(&user).await;
        let payment_method_1 = card_1.payment_method_id.clone();
        let payment_method_2 = card_2.payment_method_id.clone();

        let pc = create_passthrough_card(&user).await;

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
                        charge_request.payment_method_id == payment_method_1.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
                }
            )
            .times(1)
            .return_once(move |_| Ok(resp_1));

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                        charge_request.payment_method_id == payment_method_2.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
                }
            )
            .times(1)
            .return_once(move |_| Ok(resp_2));

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));

        let mut asa = create_example_asa(amount_cents, mcc.to_string());
        asa.token = Some(pc.token.to_string());
        let res = engine.clone().charge_from_asa_request(
            &asa,
            &vec![card_1.clone().into(), card_2.clone().into()],
            &pc,
            &user
        ).await.expect("no error");
        assert_eq!(res, AsaResponseResult::Approved);
    }


    #[test]
    async fn test_charge_user_wallet_second_card_fails() {
        crate::test_helper::general::init();
        let mut metadata = default_transaction_metadata();
        let user = create_user().await;
        let mut rtx = create_registered_transaction(&user, &metadata).await;

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc1 = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();


        let mut card_1 = create_wallet_with_rule(&user).await;
        let mut card_2 = create_wallet_with_rule(&user).await;
        let payment_method_1 = card_1.payment_method_id.clone();
        let payment_method_2 = card_2.payment_method_id.clone();

        let amount_cents = metadata.amount_cents;
        let mcc = metadata.mcc.clone();
        let mcc2 = metadata.mcc.clone();

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
                        charge_request.payment_method_id == payment_method_1.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
                }
            )
            .times(1)
            .return_once(move |_| Ok(resp_1));

        footprint_mock.expect_proxy_adyen_payment_request()
            .withf(
                move |charge_request| {
                        charge_request.payment_method_id == payment_method_2.to_string()
                        && charge_request.customer_public_id == &user.public_id.to_string()
                }
            )
            .times(1)
            .return_once( move |_| Ok(resp_2));

        let footprint_service = Arc::new(footprint_mock);
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));

        let (res, ledger) = engine.clone().charge_wallet(
            &user,
            &vec![card_1.clone().into(), card_2.clone().into()],
            &metadata,
            &rtx
        ).await.expect("NO error");
        assert_eq!(ChargeEngineResult::Denied, res);
    }

    #[test]
    async fn test_register_transaction() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let metadata = default_transaction_metadata();
        let footprint_service = Arc::new(MockFootprintServiceTrait::new());
        let user_service = Arc::new(UserService::new_with_services(footprint_service.clone()));
        let ledger_serivice = Arc::new(LedgerService::new());

        let engine = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_serivice.clone(),
            footprint_service
        ));
        let rtx = engine.clone().register_transaction_only(
            &user,
            &metadata
        ).await.expect("ok");
        assert_eq!(rtx.user_id, user.id);
        assert_eq!(rtx.amount_cents, metadata.amount_cents);
        assert_eq!(rtx.mcc, metadata.mcc);
        assert_eq!(rtx.memo, metadata.memo);
    }


    async fn create_registered_transaction(
        user: &UserModel,
        metadata: &TransactionMetadata
    ) -> RegisteredTransactionModel {
        let charge_dao = Arc::new(ChargeDao::new());
        let user_clone = user.clone();
        let metadata_clone = metadata.clone();

        let rtx: RegisteredTransactionModel = transactional::<_, DataError, _>(move |conn|
            Box::pin(async move {
                charge_dao.clone().insert_registered_transaction(conn, &InsertableRegisteredTransaction {
                    user_id: user_clone.id,
                    memo: &metadata_clone.memo,
                    amount_cents: metadata_clone.amount_cents,
                    mcc: &metadata_clone.mcc,
                }).await
            })).await.unwrap().into();
        rtx
    }


}