#[cfg(test)]
mod service_tests {
    use std::sync::Arc;
    use actix_web::test;
    use crate::ledger::constant::ChargeStatus;
    use crate::ledger::entity::TransactionLedger;
    use crate::ledger::error::LedgerError;
    use crate::ledger::model::RegisteredTransactionModel;
    use crate::ledger::service::{LedgerService, LedgerServiceTrait};
    use crate::test_helper::ledger::{create_mock_registered_transaction, create_mock_success_inner_charge, create_mock_success_outer_charge, default_transaction_metadata};
    use crate::test_helper::passthrough_card::{create_mock_passthrough_card, create_passthrough_card};
    use crate::test_helper::user::create_user;
    use crate::test_helper::wallet::{create_mock_wallet, create_wallet};
    use crate::wallet::model::WalletModel;

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
        let wallet = create_wallet(&user).await;
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
        let wallet = create_wallet(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = create_mock_registered_transaction(&metadata);
        let error = ledger.clone().register_failed_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("ok");
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }

    #[test]
    async fn test_register_failed_inner_card_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_mock_wallet();
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }

    #[test]
    async fn test_register_success_inner() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet(&user).await;
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
        let wallet = create_wallet(&user).await;
        let metadata = default_transaction_metadata();
        let ledger = Arc::new(LedgerService::new());
        let rtx = create_mock_registered_transaction(&metadata);
        let error = ledger.clone().register_successful_inner_charge(
            &rtx,
            &metadata,
            &wallet
        ).await.expect_err("ok");
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }

    #[test]
    async fn test_register_success_inner_card_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_mock_wallet();
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }

    #[test]
    async fn test_register_success_inner_throws_dupe() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet(&user).await;
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);

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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
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
        let wallet = create_wallet(&user).await;
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
        let wallet = create_wallet(&user).await;
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
        let wallet = create_wallet(&user).await;
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }

    #[test]
    async fn test_register_full_inner_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet(&user).await;
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }

    #[test]
    async fn test_register_full_outer_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet(&user).await;
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
        assert_eq!(LedgerError::UnexpectedLedgerError("test".into()), error);
    }
}