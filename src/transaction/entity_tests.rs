#[cfg(test)]
mod entity_tests {
    use lithic_client::models::Card;
    use uuid::Uuid;
    use crate::passthrough_card::entity::{create_test_lithic_card, PassthroughCard};
    use crate::schema::inner_charge_ledger::dsl::inner_charge_ledger;
    use crate::schema::outer_charge_ledger::dsl::outer_charge_ledger;
    use crate::test_helper::initialize_user;
    use crate::transaction::constant::ChargeStatus;
    use crate::transaction::entity::{InsertableInnerChargeLedger, InnerChargeLedger, InsertableOuterChargeLedger, OuterChargeLedger, RegisteredTransaction, InsertableRegisteredTransaction, TransactionLedger, InsertableTransactionLedger};
    use crate::wallet::entity::{Wallet, NewCard};

    const TEST_MEMO: &str = "Test charge";
    const TEST_MCC: &str = "0000";
    const TEST_AMOUNT: i32 = 10000;
    const TEST_TXN_ID: Uuid = Uuid::from_u128(0x9cb4cf49_5c3d_4647_83b0_8f3515da7be1);

    #[actix_web::test]
    async fn test_registered_txn_create() {
        crate::test::init();
        let user = initialize_user();
        let txn_res = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        );

        let txn = txn_res.expect("transaction should be ok");
        assert_eq!(txn.user_id, user.id);
        assert_eq!(txn.memo, TEST_MEMO);
        assert_eq!(txn.amount_cents, TEST_AMOUNT);
        assert_eq!(txn.mcc, TEST_MCC);
        assert_eq!(txn.transaction_id, TEST_TXN_ID);
        txn.delete_self().expect("transaction should delete");

        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_registered_txn_create_fails_dupe() {
        crate::test::init();
        let user = initialize_user();
        let txn_res = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        );

        let txn = txn_res.expect("transaction should be ok");
        assert_eq!(txn.user_id, user.id);
        assert_eq!(txn.memo, TEST_MEMO);
        assert_eq!(txn.amount_cents, TEST_AMOUNT);
        assert_eq!(txn.mcc, TEST_MCC);
        assert_eq!(txn.transaction_id, TEST_TXN_ID);
        let err = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect_err("Expect api error");
        txn.delete_self().expect("transaction should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_inner_charge_creates() {
        crate::test::init();
        let user = initialize_user();
        let rtx = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect("transaction should be ok");


        let (card, ca) = Wallet::create_test_wallet_in_db(
            user.id,
            1
        ).expect("should create");

        let inner_charge = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail.as_str(),
                is_success: None

            }
        ).expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Fail.as_str());
        assert_eq!(inner_charge.is_success, None);
        assert_eq!(inner_charge.registered_transaction_id, rtx.transaction_id);

        inner_charge.delete_self().expect("should delete");
        card.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_inner_charge_creates_several() {
        crate::test::init();
        let user = initialize_user();
        let rtx = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect("transaction should be ok");


        let (card, ca) = Wallet::create_test_wallet_in_db(
            user.id,
            1
        ).expect("should create");

        let inner_charge1 = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail.as_str(),
                is_success: None

            }
        ).expect("should create");

        assert_eq!(inner_charge1.user_id, user.id);
        assert_eq!(inner_charge1.wallet_card_id, card.id);
        assert_eq!(inner_charge1.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge1.status, ChargeStatus::Fail.as_str());
        assert_eq!(inner_charge1.is_success, None);
        assert_eq!(inner_charge1.registered_transaction_id, rtx.transaction_id);

        let inner_charge2 = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail.as_str(),
                is_success: None

            }
        ).expect("should create");

        assert_eq!(inner_charge2.user_id, user.id);
        assert_eq!(inner_charge2.wallet_card_id, card.id);
        assert_eq!(inner_charge2.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge2.status, ChargeStatus::Fail.as_str());
        assert_eq!(inner_charge2.is_success, None);
        assert_eq!(inner_charge2.registered_transaction_id, rtx.transaction_id);

        inner_charge1.delete_self().expect("should delete");
        inner_charge2.delete_self().expect("should delete");
        card.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_inner_charge_fails_dupe_success() {
        crate::test::init();
        let user = initialize_user();
        let rtx = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect("transaction should be ok");


        let (card, ca) = Wallet::create_test_wallet_in_db(
            user.id,
            1
        ).expect("should create");

        let inner_charge1 = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect("should create");

        assert_eq!(inner_charge1.user_id, user.id);
        assert_eq!(inner_charge1.wallet_card_id, card.id);
        assert_eq!(inner_charge1.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge1.status, ChargeStatus::Success.as_str());
        assert_eq!(inner_charge1.is_success, Some(true));
        assert_eq!(inner_charge1.registered_transaction_id, rtx.transaction_id);

        let charge_error = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect_err("should create error");

        inner_charge1.delete_self().expect("should delete");
        card.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_inner_charge_fails_no_registered_txn() {
        crate::test::init();
        let user = initialize_user();

        let (card, ca) = Wallet::create_test_wallet_in_db(
            user.id,
            1
        ).expect("should create");

        let charge_error = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: TEST_TXN_ID,
                user_id: user.id,
                wallet_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect_err("should create error");


        card.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_outer_charge_success() {
        crate::test::init();
        let user = initialize_user();
        let rtx = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect("transaction should be ok");


        let card = PassthroughCard::create_from_api_card(
        &create_test_lithic_card(
                "09".to_string(),
                "2026".to_string(),
                "1234".to_string(),
                Uuid::new_v4()
            ),
            &user
        ).expect("should create card");

        let outer_charge = OuterChargeLedger::insert(
            InsertableOuterChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail.as_str(),
                is_success: None

            }
        ).expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Fail.as_str());
        assert_eq!(outer_charge.is_success, None);
        assert_eq!(outer_charge.registered_transaction_id, rtx.transaction_id);

        outer_charge.delete_self().expect("should delete");
        card.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_outer_charge_fails_no_registered_txn() {
        crate::test::init();
        let user = initialize_user();

        let card = PassthroughCard::create_from_api_card(
            &create_test_lithic_card(
                "09".to_string(),
                "2026".to_string(),
                "1234".to_string(),
                Uuid::new_v4()
            ),
            &user
        ).expect("should create card");

        let charge_error = OuterChargeLedger::insert(
            InsertableOuterChargeLedger {
                registered_transaction_id: TEST_TXN_ID,
                user_id: user.id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Fail.as_str(),
                is_success: None

            }
        ).expect_err("should be error");

        card.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_outer_charge_fails_dupe_registered_txn() {
        crate::test::init();
        let user = initialize_user();
        let rtx = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect("transaction should be ok");


        let card = PassthroughCard::create_from_api_card(
            &create_test_lithic_card(
                "09".to_string(),
                "2026".to_string(),
                "1234".to_string(),
                Uuid::new_v4()
            ),
            &user
        ).expect("should create card");

        let outer_charge = OuterChargeLedger::insert(
            InsertableOuterChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success.as_str());
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.transaction_id);

        let dupe_error = OuterChargeLedger::insert(
            InsertableOuterChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                passthrough_card_id: card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect_err("Should be err");

        outer_charge.delete_self().expect("should delete");
        card.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }

    #[actix_web::test]
    async fn test_transaction_ledger_ok() {
        crate::test::init();
        let user = initialize_user();
        let rtx = RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: TEST_TXN_ID,
                memo: TEST_MEMO.to_string(),
                amount_cents: TEST_AMOUNT,
                mcc: TEST_MCC.to_string()
            }
        ).expect("transaction should be ok");


        let (wallet_card, ca) = Wallet::create_test_wallet_in_db(
            user.id,
            1
        ).expect("should create");

        let inner_charge = InnerChargeLedger::insert(
            InsertableInnerChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                wallet_card_id: wallet_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect("should create");

        assert_eq!(inner_charge.user_id, user.id);
        assert_eq!(inner_charge.wallet_card_id, wallet_card.id);
        assert_eq!(inner_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(inner_charge.status, ChargeStatus::Success.as_str());
        assert_eq!(inner_charge.is_success, Some(true));
        assert_eq!(inner_charge.registered_transaction_id, rtx.transaction_id);

        let outer_card = PassthroughCard::create_from_api_card(
            &create_test_lithic_card(
                "09".to_string(),
                "2026".to_string(),
                "1234".to_string(),
                Uuid::new_v4()
            ),
            &user
        ).expect("should create card");

        let outer_charge = OuterChargeLedger::insert(
            InsertableOuterChargeLedger {
                registered_transaction_id: rtx.transaction_id,
                user_id: rtx.user_id,
                passthrough_card_id: outer_card.id,
                amount_cents: TEST_AMOUNT,
                status: ChargeStatus::Success.as_str(),
                is_success: Some(true)

            }
        ).expect("should create");

        assert_eq!(outer_charge.user_id, user.id);
        assert_eq!(outer_charge.passthrough_card_id, outer_card.id);
        assert_eq!(outer_charge.amount_cents, TEST_AMOUNT);
        assert_eq!(outer_charge.status, ChargeStatus::Success.as_str());
        assert_eq!(outer_charge.is_success, Some(true));
        assert_eq!(outer_charge.registered_transaction_id, rtx.transaction_id);

        let final_tx = TransactionLedger::insert(
            InsertableTransactionLedger {
                transaction_id: rtx.transaction_id,
                inner_charge_ledger_id: inner_charge.id,
                outer_charge_ledger_id: outer_charge.id
            }
        ).expect("should create txn");

        assert_eq!(final_tx.transaction_id, rtx.transaction_id);
        assert_eq!(final_tx.inner_charge_ledger_id, inner_charge.id);
        assert_eq!(final_tx.outer_charge_ledger_id, outer_charge.id);

        final_tx.delete_self().expect("should delete");
        outer_charge.delete_self().expect("should delete");
        outer_card.delete_self().expect("should delete");
        inner_charge.delete_self().expect("should delete");
        wallet_card.delete_self().expect("should delete");
        ca.delete_self().expect("should delete");
        rtx.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }
}