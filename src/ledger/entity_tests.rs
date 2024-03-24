#[cfg(test)]
mod entity_tests {
    use crate::passthrough_card::entity::PassthroughCard;
    use crate::test_helper::passthrough_card::create_mock_lithic_card;
    use crate::test_helper::user::create_user;
    use crate::ledger::constant::ChargeStatus;
    use crate::ledger::entity::{InsertableInnerChargeLedger, InnerChargeLedger, InsertableOuterChargeLedger, OuterChargeLedger, RegisteredTransaction, InsertableRegisteredTransaction, TransactionLedger, InsertableTransactionLedger};
    use crate::test_helper::wallet::create_test_wallet_in_db;
    use crate::wallet::entity::Wallet;
    use actix_web::test;

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
        txn.delete_self().await.expect("ledger should delete");

        user.delete_self().await.expect("should delete");
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
        ).await.expect_err("Expect api error");
        txn.delete_self().await.expect("ledger should delete");
        user.delete_self().await.expect("should delete");
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


        let (card, ca) = create_test_wallet_in_db(
            user.id,
            1
        ).await.expect("should create");

        let inner_charge = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None

            }
        ).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Fail);
        assert_eq!(inner_charge.is_success, None);
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);

        inner_charge.delete_self().await.expect("should delete");
        card.delete_self().await.expect("should delete");
        ca.delete_self().await.expect("should delete");
        rtx.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
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


        let (card, ca) = create_test_wallet_in_db(
            user.id,
            1
        ).await.expect("should create");

        let inner_charge1 = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None

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
                is_success: None

            }
        ).await.expect("should create");

        assert_eq!(inner_charge2.user_id, user.id);
        assert_eq!(inner_charge2.wallet_card_id, card.id);
        assert_eq!(inner_charge2.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge2.status, ChargeStatus::Fail);
        assert_eq!(inner_charge2.is_success, None);
        assert_eq!(inner_charge2.registered_transaction_id, rtx.id);

        inner_charge1.delete_self().await.expect("should delete");
        inner_charge2.delete_self().await.expect("should delete");
        card.delete_self().await.expect("should delete");
        ca.delete_self().await.expect("should delete");
        rtx.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
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


        let (card, ca) = create_test_wallet_in_db(
            user.id,
            1
        ).await.expect("should create");

        let inner_charge1 = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

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
                is_success: Some(true)

            }
        ).await.expect_err("should create error");

        inner_charge1.delete_self().await.expect("should delete");
        card.delete_self().await.expect("should delete");
        ca.delete_self().await.expect("should delete");
        rtx.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }

    #[test]
    async fn test_inner_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;

        let (card, ca) = create_test_wallet_in_db(
            user.id,
            1
        ).await.expect("should create");

        let charge_error = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: 1,
                user_id: user.id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

            }
        ).await.expect_err("should create error");


        card.delete_self().await.expect("should delete");
        ca.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
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


        let card = PassthroughCard::create_from_api_card(
            &create_mock_lithic_card(),
            &user
        ).await.expect("should create card");

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

        outer_charge.delete_self().await.expect("should delete");
        card.delete_self().await.expect("should delete");
        rtx.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }

    #[test]
    async fn test_outer_charge_fails_no_registered_txn() {
        crate::test_helper::general::init();
        let user = create_user().await;

        let card = PassthroughCard::create_from_api_card(
            &create_mock_lithic_card(),
            &user
        ).await.expect("should create card");

        let charge_error = OuterChargeLedger::insert(
            &InsertableOuterChargeLedger {
                registered_transaction_id: 1,
                user_id: user.id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail,
                is_success: None

            }
        ).await.expect_err("should be error");

        card.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }

    #[test]
    async fn test_outer_charge_fails_dupe_registered_txn() {
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


        let card = PassthroughCard::create_from_api_card(
            &create_mock_lithic_card(),
            &user
        ).await.expect("should create card");

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

        outer_charge.delete_self().await.expect("should delete");
        card.delete_self().await.expect("should delete");
        rtx.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }

    #[test]
    async fn test_transaction_ledger_ok() {
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


        let (wallet_card, ca) = create_test_wallet_in_db(
            user.id,
            1
        ).await.expect("should create");

        let inner_charge = InnerChargeLedger::insert(
            &InsertableInnerChargeLedger {
                registered_transaction_id: rtx.id,
                user_id: rtx.user_id,
                wallet_card_id: wallet_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success,
                is_success: Some(true)

            }
        ).await.expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, wallet_card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Success);
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.registered_transaction_id, rtx.id);

        let outer_card = PassthroughCard::create_from_api_card(
            &create_mock_lithic_card(),
            &user
        ).await.expect("should create card");

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
                outer_charge_ledger_id: outer_charge.id
            }
        ).await.expect("should create txn");

        assert_eq!(final_tx.registered_transaction_id, rtx.id);
        assert_eq!(final_tx.inner_charge_ledger_id, inner_charge.id);
        assert_eq!(final_tx.outer_charge_ledger_id, outer_charge.id);

        final_tx.delete_self().await.expect("should delete");
        outer_charge.delete_self().await.expect("should delete");
        outer_card.delete_self().await.expect("should delete");
        inner_charge.delete_self().await.expect("should delete");
        wallet_card.delete_self().await.expect("should delete");
        ca.delete_self().await.expect("should delete");
        rtx.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }
}