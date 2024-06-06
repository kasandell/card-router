#[cfg(test)]
mod dao_tests {
    use std::sync::Arc;
    use crate::passthrough_card::model::PassthroughCardModel as PassthroughCard;
    use crate::test_helper::passthrough_card::{create_mock_lithic_card, create_passthrough_card};
    use crate::test_helper::user::create_user;
    use crate::charge::constant::ChargeStatus;
    use crate::charge::entity::{InsertableWalletCardCharge, WalletCardCharge, InsertablePassthroughCardCharge, PassthroughCardCharge, RegisteredTransaction, InsertableRegisteredTransaction, SuccessfulEndToEndCharge, InsertableSuccessfulEndToEndCharge, InsertableExpectedWalletChargeReference};
    use crate::wallet::model::WalletModel as Wallet;
    use crate::test_helper::wallet::create_wallet;
    use actix_web::test;
    use uuidv7::create;
    use crate::error::data_error::DataError;
    use crate::charge::dao::{ChargeDao, ChargeDaoTrait};
    use crate::charge::error::ChargeError;
    use crate::util::transaction::transactional;

    const TEST_MEMO: &str = "Test charge";
    const TEST_MCC: &str = "0000";
    const TEST_AMOUNT: i32 = 10000;

    #[test]
    async fn test_registered_txn_create() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let dc = dao.clone();
        let txn_res: Result<RegisteredTransaction, DataError> = transactional(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await;

        let txn = txn_res.expect("ledger should be ok");
        assert_eq!(txn.user_id, user.id);
        assert_eq!(txn.memo, TEST_MEMO);
        assert_eq!(txn.amount_cents, TEST_AMOUNT);
        assert_eq!(txn.mcc, TEST_MCC);
        let get_by_id = dao.clone().get_registered_transaction(txn.id).await.expect("finds");
        let get_by_txn = dao.clone().get_registered_transaction_by_transaction_id(&txn.transaction_id).await.expect("finds");
        assert_eq!(txn.id, get_by_id.id);
        assert_eq!(txn.id, get_by_txn.id);
    }

    #[test]
    async fn test_inner_charge_creates() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");


        let card = create_wallet(
            &user
        ).await;


        dc = dao.clone();
        let inner_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");
            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: None,
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Fail,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.resolved_charge_status, ChargeStatus::Fail);
        assert_eq!(inner_charge.is_success, None);
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);
        let get_by_id = dao.clone().get_wallet_charge_by_id(inner_charge.id).await.expect("ok");
        assert_eq!(get_by_id.id, inner_charge.id);
        let get_by_txn = dao.clone().get_wallet_charges_by_registered_transaction(rtx.id).await.expect("ok");
        assert_eq!(1, get_by_txn.len());
        assert_eq!(inner_charge.id, get_by_txn[0].id);
    }

    #[test]
    async fn test_inner_charge_creates_several() {
        crate::test_helper::general::init();
        let dao = Arc::new(ChargeDao::new());
        let user = create_user().await;
        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("creates");


        let card = create_wallet(&user).await;

        dc = dao.clone();
        let inner_charge1 = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");
            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: None,
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Fail,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(inner_charge1.user_id, user.id);
        assert_eq!(inner_charge1.wallet_card_id, card.id);
        assert_eq!(inner_charge1.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge1.resolved_charge_status, ChargeStatus::Fail);
        assert_eq!(inner_charge1.is_success, None);
        assert_eq!(inner_charge1.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let inner_charge2 = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: None,
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Fail,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(inner_charge2.user_id, user.id);
        assert_eq!(inner_charge2.wallet_card_id, card.id);
        assert_eq!(inner_charge2.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge2.resolved_charge_status, ChargeStatus::Fail);
        assert_eq!(inner_charge2.is_success, None);
        assert_eq!(inner_charge2.registered_transaction_id, rtx.id);
        let get_by_txn = dao.clone().get_wallet_charges_by_registered_transaction(rtx.id).await.expect("ok");
        assert_eq!(2, get_by_txn.len());
        let error = dao.clone().get_successful_wallet_charge_by_registered_transaction(rtx.id).await.expect_err("should not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }

    #[test]
    #[ignore]
    // transactions failing
    async fn test_inner_charge_fails_dupe_success() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC,
                }
            ).await
        })).await.expect("ledger should be ok");


        let card = create_wallet(&user).await;

        dc = dao.clone();
        let inner_charge1 = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");
            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(inner_charge1.user_id, user.id);
        assert_eq!(inner_charge1.wallet_card_id, card.id);
        assert_eq!(inner_charge1.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge1.resolved_charge_status, ChargeStatus::Success);
        assert_eq!(inner_charge1.is_success, Some(true));
        assert_eq!(inner_charge1.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let charge_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect_err("should create error");
        assert_eq!(DataError::Conflict("test".into()), charge_error);
        //let get_by_success = dao.clone().get_successful_wallet_charge_by_registered_transaction(rtx.id).await.expect("should find");
        //assert_eq!(get_by_success.id, inner_charge1.id);
    }

    #[test]
    async fn test_inner_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let card = create_wallet(&user).await;

        let dc = dao.clone();
        let charge_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: 1,
                    user_id: user.id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: 0,
                    resolved_charge_status: ChargeStatus::Fail,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect_err("should create error");
        assert_eq!(DataError::Unexpected("test".into()), charge_error);
    }

    #[test]
    async fn test_outer_charge_success() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        let error = dao.clone().get_passthrough_card_charge_by_registered_transaction(rtx.id).await.expect_err("should find");
        assert_eq!(DataError::NotFound("test".into()), error);

        let card = create_passthrough_card(&user).await;

        dc = dao.clone();
        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Fail,
                    is_success: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Fail);
        assert_eq!(outer_charge.is_success, None);
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);
        let get_by_id = dao.clone().get_passthrough_card_charge_by_id(outer_charge.id).await.expect("should find");
        assert_eq!(get_by_id.id, outer_charge.id);
        let get_by_rtx = dao.clone().get_passthrough_card_charge_by_registered_transaction(rtx.id).await.expect("should find");
        assert_eq!(get_by_rtx.id, outer_charge.id);
    }

    #[test]
    async fn test_outer_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let card = create_passthrough_card(&user).await;

        let charge_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dao.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: -1,
                    user_id: user.id,
                    passthrough_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Fail,
                    is_success: None,
                }
            ).await
        })).await.expect_err("should be error");
        assert_eq!(DataError::Unexpected("test".into()), charge_error);
    }


    #[test]
    async fn test_outer_charge_fails_dupe_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let card = create_passthrough_card(&user).await;

        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        dc = dao.clone();
        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)

                }
            ).await
        })).await.expect("ok");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let dupe_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)

                }
            ).await
        })).await.expect_err("Should be err");
        assert_eq!(DataError::Conflict("test".into()), dupe_error);
    }

    #[test]
    #[ignore]
    // transactions failing
    async fn test_transaction_ledger_ok() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;

        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        dc = dao.clone();
        let inner_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: 0,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, wallet_card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.resolved_charge_status, ChargeStatus::Success);
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: outer_card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)

                }
            ).await
        })).await.expect("ok");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, outer_card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let final_tx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await
        })).await.expect("ok");

        assert_eq!(final_tx.registered_transaction_id, rtx.id);
        assert_eq!(final_tx.wallet_card_charge_id, inner_charge.id);
        assert_eq!(final_tx.passthrough_card_charge_id, outer_charge.id);
        let tx_by_id = dao.clone().get_successful_end_to_end_charge_by_id(final_tx.id).await.expect("finds");
        let tx_by_rtx = dao.clone().get_successful_end_to_end_charge_by_registered_transaction_id(rtx.id).await.expect("finds");
        assert_eq!(final_tx.id, tx_by_id.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_id.registered_transaction_id);
        assert_eq!(final_tx.id, tx_by_rtx.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_rtx.registered_transaction_id);
    }

    #[test]
    async fn test_transaction_ledger_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;

        let mut dc = dao.clone();
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("creates");

        dc = dao.clone();
        let inner_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, wallet_card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.resolved_charge_status, ChargeStatus::Success);
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: outer_card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)

                }
            ).await
        })).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, outer_card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        dc = dao.clone();
        let final_tx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await
        })).await.expect("should create txn");

        assert_eq!(final_tx.registered_transaction_id, rtx.id);
        assert_eq!(final_tx.wallet_card_charge_id, inner_charge.id);
        assert_eq!(final_tx.passthrough_card_charge_id, outer_charge.id);
        let tx_by_id = dao.clone().get_successful_end_to_end_charge_by_id(final_tx.id).await.expect("finds");
        let tx_by_rtx = dao.clone().get_successful_end_to_end_charge_by_registered_transaction_id(rtx.id).await.expect("finds");
        assert_eq!(final_tx.id, tx_by_id.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_id.registered_transaction_id);
        assert_eq!(final_tx.id, tx_by_rtx.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_rtx.registered_transaction_id);

        dc = dao.clone();
        let error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await
        })).await.expect_err("should error");
        assert_eq!(DataError::Conflict("test".into()), error);
    }


    #[test]
    async fn test_transaction_ledger_throws_dupe_outer() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;

        let mut dc = dao.clone();
        transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let rtx = dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await.expect("ledger should be ok");

            let rtx_2 = dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await.expect("ok");
            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            let inner_charge = dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await.expect("should create");

            let outer_charge = dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: outer_card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)
                }
            ).await.expect("should create");

            let outer_charge_2 = dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx_2.id,
                    user_id: rtx_2.user_id,
                    passthrough_card_id: outer_card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)
                }
            ).await.expect("should create");

            let final_tx = dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await.expect("should create txn");

            let error = dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx_2.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge_2.id,
                }
            ).await.expect_err("should create conflict");
            assert_eq!(DataError::Conflict("test".into()), error);
            Ok(())
        })).await.expect_err("should cause rollback");
    }


    #[test]
    async fn test_transaction_ledger_throws_dupe_inner() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let dao = Arc::new(ChargeDao::new());
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;
        let mut dc = dao.clone();
        transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let rtx = dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await.expect("ledger should be ok");

            let rtx_2 = dc.clone().insert_registered_transaction(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await.expect("ledger should be ok");

            let expected = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            let inner_charge = dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await.expect("should create");

            let expected_2 = dc.clone().insert_expected_wallet_charge_reference(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates expect");

            let inner_charge_2 = dc.clone().insert_wallet_charge(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx_2.id,
                    user_id: rtx_2.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: expected_2.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await.expect("should create");


            let outer_charge = dc.clone().insert_passthrough_card_charge(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: outer_card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Success,
                    is_success: Some(true)

                }
            ).await.expect("should create");

            let final_tx = dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await.expect("should create txn");

            let error = dc.clone().insert_successful_end_to_end_charge(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx_2.id,
                    wallet_card_charge_id: inner_charge_2.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await.expect_err("should create conflict");
            assert_eq!(DataError::Conflict("test".into()), error);
            Ok(())
        })).await.expect_err("should trigger rollback");
    }
}