#[cfg(test)]
mod entity_tests {
    use crate::passthrough_card::model::PassthroughCardModel as PassthroughCard;
    use crate::test_helper::passthrough_card::{create_mock_lithic_card, create_passthrough_card};
    use crate::test_helper::user::create_user;
    use crate::charge::constant::ChargeStatus;
    use crate::charge::entity::{InsertableWalletCardCharge, WalletCardCharge, InsertablePassthroughCardCharge, PassthroughCardCharge, RegisteredTransaction, InsertableRegisteredTransaction, SuccessfulEndToEndCharge, InsertableSuccessfulEndToEndCharge, ExpectedWalletChargeReference, InsertableExpectedWalletChargeReference};
    use crate::wallet::model::WalletModel as Wallet;
    use crate::test_helper::wallet::create_wallet;
    use actix_web::test;
    use uuidv7::create;
    use crate::error::data_error::DataError;
    use crate::util::transaction::transactional;

    const TEST_MEMO: &str = "Test charge";
    const TEST_MCC: &str = "0000";
    const TEST_AMOUNT: i32 = 10000;

    #[test]
    async fn test_registered_txn_create() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let txn_res = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
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
        let get_by_id = RegisteredTransaction::get(txn.id).await.expect("finds");
        let get_by_txn = RegisteredTransaction::get_by_transaction_id(&txn.transaction_id).await.expect("finds");
        assert_eq!(txn.id, get_by_id.id);
        assert_eq!(txn.id, get_by_txn.id);
    }

    // no longer  possible
    async fn test_registered_txn_create_fails_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let txn_res = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
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
        let err = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect_err("Expect data error");
        assert_eq!(DataError::Conflict("test".into()), err);
    }

    #[test]
    async fn test_inner_charge_creates() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
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

        let inner_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let reference = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("should give reference");
            WalletCardCharge::insert(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: None,
                    rule_id: None,
                    expected_wallet_charge_reference_id: reference.id,
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
        let get_by_id = WalletCardCharge::get_by_id(inner_charge.id).await.expect("ok");
        assert_eq!(get_by_id.id, inner_charge.id);
        let get_by_txn = WalletCardCharge::get_wallet_card_charges_by_registered_transaction(rtx.id).await.expect("ok");
        assert_eq!(1, get_by_txn.len());
        assert_eq!(inner_charge.id, get_by_txn[0].id);
    }

    #[test]
    async fn test_inner_charge_creates_several() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");


        let card = create_wallet(&user).await;

        let inner_charge1 = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let reference = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("should create");
            
            WalletCardCharge::insert(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: None,
                    rule_id: None,
                    expected_wallet_charge_reference_id: reference.id,
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

        let inner_charge2 = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let reference = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("should create");

            WalletCardCharge::insert(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: None,
                    rule_id: None,
                    expected_wallet_charge_reference_id: reference.id,
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
        let get_by_txn = WalletCardCharge::get_wallet_card_charges_by_registered_transaction(rtx.id).await.expect("ok");
        assert_eq!(2, get_by_txn.len());
        let error = WalletCardCharge::get_successful_wallet_card_charge_by_registered_transaction(rtx.id).await.expect_err("should not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }

    #[test]
    #[ignore]
    //transactions not working
    async fn test_inner_charge_fails_dupe_success() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
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

        let inner_charge1 = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let reference = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("should create");

            WalletCardCharge::insert(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: reference.id,
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

        let charge_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let reference = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("should create");

            WalletCardCharge::insert(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: reference.id,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                }
            ).await
        })).await.expect_err("should be an error");


        assert_eq!(DataError::Conflict("test".into()), charge_error);
        let get_by_success = WalletCardCharge::get_successful_wallet_card_charge_by_registered_transaction(rtx.id).await.expect("should find");
        assert_eq!(get_by_success.id, inner_charge1.id);
    }

    #[test]
    async fn test_inner_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_wallet(&user).await;

        let charge_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            WalletCardCharge::insert(
                conn,
                &InsertableWalletCardCharge {
                    registered_transaction_id: 1,
                    user_id: user.id,
                    wallet_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    resolved_charge_status: ChargeStatus::Success,
                    psp_reference: None,
                    returned_reference: None,
                    returned_charge_status: None,
                    is_success: Some(true),
                    rule_id: None,
                    expected_wallet_charge_reference_id: 0
                }
            ).await
        })).await.expect_err("should create error");
        assert_eq!(DataError::Unexpected("test".into()), charge_error);
    }

    #[test]
    async fn test_outer_charge_success() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        let error = PassthroughCardCharge::get_outer_charge_by_registered_transaction(rtx.id).await.expect_err("should find");
        assert_eq!(DataError::NotFound("test".into()), error);

        let card = create_passthrough_card(&user).await;

        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            PassthroughCardCharge::insert(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Fail,
                    is_success: None
                }
            ).await
        })).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Fail);
        assert_eq!(outer_charge.is_success, None);
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);
        let get_by_id = PassthroughCardCharge::get_by_id(outer_charge.id).await.expect("should find");
        assert_eq!(get_by_id.id, outer_charge.id);
        let get_by_rtx = PassthroughCardCharge::get_outer_charge_by_registered_transaction(rtx.id).await.expect("should find");
        assert_eq!(get_by_rtx.id, outer_charge.id);
    }

    #[test]
    async fn test_outer_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;

        let charge_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            PassthroughCardCharge::insert(
                conn,
                &InsertablePassthroughCardCharge {
                    registered_transaction_id: -1,
                    user_id: user.id,
                    passthrough_card_id: card.id,
                    amount_cents: TEST_AMOUNT,
                    status: ChargeStatus::Fail,
                    is_success: None
                }
            ).await
        })).await.expect_err("should be error");
        assert_eq!(DataError::Unexpected("test".into()), charge_error);
    }


    #[test]
    async fn test_outer_charge_fails_dupe_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;

        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            PassthroughCardCharge::insert(
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
        })).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        let dupe_error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            PassthroughCardCharge::insert(
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
    async fn test_transaction_ledger_ok() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;

        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        let inner_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates ok");

            WalletCardCharge::insert(
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

        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            PassthroughCardCharge::insert(
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

        let final_tx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            SuccessfulEndToEndCharge::insert(
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
        let tx_by_id = SuccessfulEndToEndCharge::get_by_id(final_tx.id).await.expect("finds");
        let tx_by_rtx = SuccessfulEndToEndCharge::get_by_registered_transaction_id(rtx.id).await.expect("finds");
        assert_eq!(final_tx.id, tx_by_id.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_id.registered_transaction_id);
        assert_eq!(final_tx.id, tx_by_rtx.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_rtx.registered_transaction_id);
    }


    #[test]
    async fn test_transaction_ledger_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;

        let rtx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            RegisteredTransaction::insert(
                conn,
                &InsertableRegisteredTransaction {
                    user_id: user.id,
                    memo: TEST_MEMO,
                    amount_cents: TEST_AMOUNT,
                    mcc: TEST_MCC
                }
            ).await
        })).await.expect("ledger should be ok");

        let inner_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            let expected = ExpectedWalletChargeReference::insert(
                conn,
                &InsertableExpectedWalletChargeReference {
                    registered_transaction_id: rtx.id,
                    user_id: rtx.user_id,
                    wallet_card_id: wallet_card.id,
                    amount_cents: TEST_AMOUNT,
                }
            ).await.expect("creates ok");

            WalletCardCharge::insert(
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

        let outer_charge = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            PassthroughCardCharge::insert(
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

        let final_tx = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            SuccessfulEndToEndCharge::insert(
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
        let tx_by_id = SuccessfulEndToEndCharge::get_by_id(final_tx.id).await.expect("finds");
        let tx_by_rtx = SuccessfulEndToEndCharge::get_by_registered_transaction_id(rtx.id).await.expect("finds");
        assert_eq!(final_tx.id, tx_by_id.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_id.registered_transaction_id);
        assert_eq!(final_tx.id, tx_by_rtx.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_rtx.registered_transaction_id);
        let error = transactional::<_, DataError, _>(move |conn| Box::pin(async move {
            SuccessfulEndToEndCharge::insert(
                conn,
                &InsertableSuccessfulEndToEndCharge {
                    registered_transaction_id: rtx.id,
                    wallet_card_charge_id: inner_charge.id,
                    passthrough_card_charge_id: outer_charge.id,
                }
            ).await
        })).await.expect_err("should throw");
        assert_eq!(DataError::Conflict("test".into()), error);
    }
}