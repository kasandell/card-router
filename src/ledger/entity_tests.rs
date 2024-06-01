#[cfg(test)]
mod entity_tests {
    use crate::passthrough_card::model::PassthroughCardModel as PassthroughCard;
    use crate::test_helper::passthrough_card::{create_mock_lithic_card, create_passthrough_card};
    use crate::test_helper::user::create_user;
    use crate::ledger::constant::ChargeStatus;
    use crate::ledger::entity::{InsertableInnerChargeLedger, InnerChargeLedger, InsertableOuterChargeLedger, OuterChargeLedger, RegisteredTransaction, InsertableRegisteredTransaction, TransactionLedger, InsertableTransactionLedger};
    use crate::wallet::model::WalletModel as Wallet;
    use crate::test_helper::wallet::create_wallet;
    use actix_web::test;
    use uuidv7::create;
    use crate::error::data_error::DataError;

    const TEST_MEMO: &str = "Test charge";
    const TEST_MCC: &str = "0000";
    const TEST_AMOUNT: i32 = 10000;

    #[test]
    async fn test_registered_txn_create() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let txn_res = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await;

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
        let txn_res = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await;

        let txn = txn_res.expect("ledger should be ok");
        assert_eq!(txn.user_id, user.id);
        assert_eq!(txn.memo, TEST_MEMO);
        assert_eq!(txn.amount_cents, TEST_AMOUNT);
        assert_eq!(txn.mcc, TEST_MCC);
        let err = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect_err("Expect data error");
        assert_eq!(DataError::Conflict("test".into()), err);
    }

    #[test]
    async fn test_inner_charge_creates() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect("ledger should be ok");


        let card = create_wallet(
            &user
        ).await;

        let inner_charge = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None,
                rule_id: None,
            }
        ).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Fail);
        assert_eq!(inner_charge.is_success, None);
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);
        let get_by_id = InnerChargeLedger::get_by_id(inner_charge.id).await.expect("ok");
        assert_eq!(get_by_id.id, inner_charge.id);
        let get_by_txn = InnerChargeLedger::get_inner_charges_by_registered_transaction(rtx.id).await.expect("ok");
        assert_eq!(1, get_by_txn.len());
        assert_eq!(inner_charge.id, get_by_txn[0].id);
    }

    #[test]
    async fn test_inner_charge_creates_several() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect("ledger should be ok");


        let card = create_wallet(&user).await;

        let inner_charge1 = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None,
                rule_id: None,
            }
        ).await.expect("should create");

        assert_eq!(inner_charge1.user_id, user.id);
        assert_eq!(inner_charge1.wallet_card_id, card.id);
        assert_eq!(inner_charge1.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge1.status, ChargeStatus::Fail);
        assert_eq!(inner_charge1.is_success, None);
        assert_eq!(inner_charge1.registered_transaction_id, rtx.id);

        let inner_charge2 = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None,
                rule_id: None,
            }
        ).await.expect("should create");

        assert_eq!(inner_charge2.user_id, user.id);
        assert_eq!(inner_charge2.wallet_card_id, card.id);
        assert_eq!(inner_charge2.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge2.status, ChargeStatus::Fail);
        assert_eq!(inner_charge2.is_success, None);
        assert_eq!(inner_charge2.registered_transaction_id, rtx.id);
        let get_by_txn = InnerChargeLedger::get_inner_charges_by_registered_transaction(rtx.id).await.expect("ok");
        assert_eq!(2, get_by_txn.len());
        let error = InnerChargeLedger::get_successful_inner_charge_by_registered_transaction(rtx.id).await.expect_err("should not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_inner_charge_fails_dupe_success() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC,
            }
        ).await.expect("ledger should be ok");


        let card = create_wallet(&user).await;

        let inner_charge1 = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true),
                rule_id: None

            }
        ).await.expect("should create");

        assert_eq!(inner_charge1.user_id, user.id);
        assert_eq!(inner_charge1.wallet_card_id, card.id);
        assert_eq!(inner_charge1.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge1.status, ChargeStatus::Success);
        assert_eq!(inner_charge1.is_success, Some(true));
        assert_eq!(inner_charge1.registered_transaction_id, rtx.id);

        let charge_error = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true),
                rule_id: None

            }
        ).await.expect_err("should create error");
        assert_eq!(DataError::Conflict("test".into()), charge_error);
        let get_by_success = InnerChargeLedger::get_successful_inner_charge_by_registered_transaction(rtx.id).await.expect("should find");
        assert_eq!(get_by_success.id, inner_charge1.id);
    }

    #[test]
    async fn test_inner_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_wallet(&user).await;

        let charge_error = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: 1,
                user_id: user.id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true),
                rule_id: None

            }
        ).await.expect_err("should create error");
        assert_eq!(DataError::Unexpected("test".into()), charge_error);
    }

    #[test]
    async fn test_outer_charge_success() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect("ledger should be ok");

        let error = OuterChargeLedger::get_outer_charge_by_registered_transaction(rtx.id).await.expect_err("should find");
        assert_eq!(DataError::NotFound("test".into()), error);

        let card = create_passthrough_card(&user).await;

        let outer_charge = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None
            }
        ).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Fail);
        assert_eq!(outer_charge.is_success, None);
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);
        let get_by_id = OuterChargeLedger::get_by_id(outer_charge.id).await.expect("should find");
        assert_eq!(get_by_id.id, outer_charge.id);
        let get_by_rtx = OuterChargeLedger::get_outer_charge_by_registered_transaction(rtx.id).await.expect("should find");
        assert_eq!(get_by_rtx.id, outer_charge.id);
    }

    #[test]
    async fn test_outer_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;

        let charge_error = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: -1,
                user_id: user.id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None
            }
        ).await.expect_err("should be error");
        assert_eq!(DataError::Unexpected("test".into()), charge_error);
    }


    #[test]
    async fn test_outer_charge_fails_dupe_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;

        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect("ledger should be ok");

        let outer_charge = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

            }
        ).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        let dupe_error = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

            }
        ).await.expect_err("Should be err");
        assert_eq!(DataError::Conflict("test".into()), dupe_error);
    }

    #[test]
    async fn test_transaction_ledger_ok() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet_card = create_wallet(&user).await;
        let outer_card = create_passthrough_card(&user).await;

        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect("ledger should be ok");

        let inner_charge = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: wallet_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true),
                rule_id: None

            }
        ).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, wallet_card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Success);
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);

        let outer_charge = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                passthrough_card_id: outer_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

            }
        ).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, outer_card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        let final_tx = TransactionLedger::insert(
            &InsertableTransactionLedger {
                registered_transaction_id: rtx.id,
                inner_charge_ledger_id: inner_charge.id,
                outer_charge_ledger_id: outer_charge.id,
                rule_id: None
            }
        ).await.expect("should create txn");

        assert_eq!(final_tx.registered_transaction_id, rtx.id);
        assert_eq!(final_tx.inner_charge_ledger_id, inner_charge.id);
        assert_eq!(final_tx.outer_charge_ledger_id, outer_charge.id);
        let tx_by_id = TransactionLedger::get_by_id(final_tx.id).await.expect("finds");
        let tx_by_rtx = TransactionLedger::get_by_registered_transaction_id(rtx.id).await.expect("finds");
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

        let rtx = RegisteredTransaction::insert(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: TEST_MEMO,
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC
            }
        ).await.expect("ledger should be ok");

        let inner_charge = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: wallet_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true),
                rule_id: None

            }
        ).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, wallet_card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Success);
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);

        let outer_charge = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                passthrough_card_id: outer_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

            }
        ).await.expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, outer_card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success);
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.id);

        let final_tx = TransactionLedger::insert(
            &InsertableTransactionLedger {
                registered_transaction_id: rtx.id,
                inner_charge_ledger_id: inner_charge.id,
                outer_charge_ledger_id: outer_charge.id,
                rule_id: None
            }
        ).await.expect("should create txn");

        assert_eq!(final_tx.registered_transaction_id, rtx.id);
        assert_eq!(final_tx.inner_charge_ledger_id, inner_charge.id);
        assert_eq!(final_tx.outer_charge_ledger_id, outer_charge.id);
        let tx_by_id = TransactionLedger::get_by_id(final_tx.id).await.expect("finds");
        let tx_by_rtx = TransactionLedger::get_by_registered_transaction_id(rtx.id).await.expect("finds");
        assert_eq!(final_tx.id, tx_by_id.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_id.registered_transaction_id);
        assert_eq!(final_tx.id, tx_by_rtx.id);
        assert_eq!(final_tx.registered_transaction_id, tx_by_rtx.registered_transaction_id);
        let error = TransactionLedger::insert(
            &InsertableTransactionLedger {
                registered_transaction_id: rtx.id,
                inner_charge_ledger_id: inner_charge.id,
                outer_charge_ledger_id: outer_charge.id,
                rule_id: None
            }
        ).await.expect_err("should throw");
        assert_eq!(DataError::Conflict("test".into()), error);
    }
}