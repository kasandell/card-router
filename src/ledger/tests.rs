#[cfg(test)]
mod service_tests {
    use std::sync::Arc;
    use actix_web::test;
    use crate::charge::model::RegisteredTransactionModel;
    use crate::charge::service::ChargeService;
    use crate::common::model::TransactionMetadata;
    use crate::error::data_error::DataError;
    use crate::footprint::service::MockFootprintServiceTrait;
    use crate::ledger::constant::{MoneyMovementDirection, MoneyMovementType};
    use crate::ledger::error::LedgerError;
    use crate::ledger::service::{LedgerService, LedgerServiceTrait};
    use crate::test_helper::charge::{create_mock_registered_transaction, default_transaction_metadata};
    use crate::test_helper::passthrough_card::{create_mock_passthrough_card, create_passthrough_card};
    use crate::test_helper::user::create_user;
    use crate::test_helper::wallet::{create_mock_wallet_with_rule, create_wallet_with_rule};
    use crate::user::model::UserModel;
    use crate::user::service::UserService;
    use crate::util::transaction::transactional;

    #[test]
    async fn test_reserve_release_wallet() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;
        let metadata = default_transaction_metadata();
        let rtx = create_registered_transaction(&user, &metadata).await;
        let mut rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let wallet_id = wallet.id;
        let amount_cents = rtx.amount_cents;
        let mut lc = ledger.clone();
        let mut charge = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            lc.clone().reserve_wallet_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.map_err(|e| DataError::Unexpected(e.into()))
        })).await.unwrap();
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.wallet_id, wallet.id);
        assert_eq!(charge.user_id, user.id);
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.money_movement_direction, MoneyMovementDirection::Credit);
        assert_eq!(charge.money_movement_type, MoneyMovementType::WalletReserve);

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        charge = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            lc.clone().release_wallet_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.map_err(|e| DataError::Unexpected(e.into()))
        })).await.unwrap();
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.user_id, user.id);
        assert_eq!(charge.wallet_id, wallet.id);
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.money_movement_direction, MoneyMovementDirection::Debit);
        assert_eq!(charge.money_movement_type, MoneyMovementType::WalletRelease);

    }

    #[test]
    async fn test_settle_wallet() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;
        let metadata = default_transaction_metadata();
        let rtx = create_registered_transaction(&user, &metadata).await;
        let rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let wallet_id = wallet.id;
        let amount_cents = rtx.amount_cents;

        let settled = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            ledger.clone().settle_wallet_card_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.map_err(|e| DataError::Unexpected(e.into()))
        })).await.unwrap();
        assert_eq!(settled.registered_transaction_id, rtx.id);
        assert_eq!(settled.amount_cents, rtx.amount_cents);
        assert_eq!(settled.user_id, user.id);
        assert_eq!(settled.wallet_id, wallet.id);
        assert_eq!(settled.registered_transaction_id, rtx.id);
        assert_eq!(settled.money_movement_direction, MoneyMovementDirection::Credit);
        assert_eq!(settled.money_movement_type, MoneyMovementType::WalletSettle);
        // todo: find the pending release created by this
    }

    #[test]
    async fn test_wallet_registered_transaction_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_wallet_with_rule(&user).await;
        let metadata = default_transaction_metadata();
        let rtx = create_mock_registered_transaction(&metadata);
        let mut rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let wallet_id = wallet.id;
        let amount_cents = rtx.amount_cents;

        let mut lc = ledger.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().reserve_wallet_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().release_wallet_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);
            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().settle_wallet_card_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);
            Ok(())
        })).await;
    }


    #[test]
    async fn test_wallet_card_not_found() {
        // TODO: generally assert that nothing is created
        crate::test_helper::general::init();
        let user = create_user().await;
        crate::test_helper::general::init();
        let user = create_user().await;
        let wallet = create_mock_wallet_with_rule();
        let metadata = default_transaction_metadata();
        let rtx = create_registered_transaction(&user, &metadata).await;
        let mut rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let wallet_id = wallet.id;
        let amount_cents = rtx.amount_cents;
        let mut lc = ledger.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().reserve_wallet_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().release_wallet_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().settle_wallet_card_amount(
                txn,
                &rtx_clone,
                wallet_id,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;
    }

    #[test]
    async fn test_reserve_release_passthrough() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let rtx = create_registered_transaction(&user, &metadata).await;
        let mut rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let amount_cents = rtx.amount_cents;
        let mut lc = ledger.clone();
        let mut card_clone = card.clone();
        let mut charge = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            lc.clone().reserve_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.map_err(|e| DataError::Unexpected(e.into()))
        })).await.unwrap();
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.user_id, user.id);
        assert_eq!(charge.passthrough_card_id, card.id);
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.money_movement_direction, MoneyMovementDirection::Debit);
        assert_eq!(charge.money_movement_type, MoneyMovementType::PassthroughCardReserve);

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        card_clone = card.clone();
        charge = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            lc.clone().release_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.map_err(|e| DataError::Unexpected(e.into()))
        })).await.unwrap();
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.amount_cents, rtx.amount_cents);
        assert_eq!(charge.user_id, user.id);
        assert_eq!(charge.passthrough_card_id, card.id);
        assert_eq!(charge.registered_transaction_id, rtx.id);
        assert_eq!(charge.money_movement_direction, MoneyMovementDirection::Credit);
        assert_eq!(charge.money_movement_type, MoneyMovementType::PassthroughCardRelease);

    }

    #[test]
    async fn test_settle_passthrough() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;
        let metadata = default_transaction_metadata();
        let rtx = create_registered_transaction(&user, &metadata).await;
        let rtx_clone = rtx.clone();
        let card_clone = card.clone();
        let ledger = Arc::new(LedgerService::new());
        let amount_cents = rtx.amount_cents;

        let settled = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            ledger.clone().settle_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.map_err(|e| DataError::Unexpected(e.into()))
        })).await.unwrap();
        assert_eq!(settled.registered_transaction_id, rtx.id);
        assert_eq!(settled.amount_cents, rtx.amount_cents);
        assert_eq!(settled.user_id, user.id);
        assert_eq!(settled.passthrough_card_id, card.id);
        assert_eq!(settled.registered_transaction_id, rtx.id);
        assert_eq!(settled.money_movement_direction, MoneyMovementDirection::Debit);
        assert_eq!(settled.money_movement_type, MoneyMovementType::PassthroughCardSettle);
        // todo: find the pending release created by this
    }

    #[test]
    async fn test_passthrough_registered_transaction_not_found() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_passthrough_card(&user).await;
        let mut card_clone = card.clone();
        let metadata = default_transaction_metadata();
        let rtx = create_mock_registered_transaction(&metadata);
        let mut rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let amount_cents = rtx.amount_cents;

        let mut lc = ledger.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().reserve_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        card_clone = card.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().release_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);
            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        card_clone = card.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().settle_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);
            Ok(())
        })).await;
    }


    #[test]
    async fn test_passthrough_card_not_found() {
        // TODO: generally assert that nothing is created
        crate::test_helper::general::init();
        let user = create_user().await;
        crate::test_helper::general::init();
        let user = create_user().await;
        let card = create_mock_passthrough_card();
        let mut card_clone = card.clone();
        let metadata = default_transaction_metadata();
        let rtx = create_registered_transaction(&user, &metadata).await;
        let mut rtx_clone = rtx.clone();
        let ledger = Arc::new(LedgerService::new());
        let amount_cents = rtx.amount_cents;

        let mut lc = ledger.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().reserve_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        card_clone = card.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().release_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;

        lc = ledger.clone();
        rtx_clone = rtx.clone();
        card_clone = card.clone();
        let _ = transactional::<_, DataError, _>(move |txn| Box::pin(async move {
            let err = lc.clone().settle_passthrough_card_amount(
                txn,
                &rtx_clone,
                &card_clone,
                amount_cents
            ).await.unwrap_err();
            assert_eq!(LedgerError::Unexpected("test".into()), err);

            Ok(())
        })).await;
    }


    // TODO: dupe tests?

    async fn create_registered_transaction(user: &UserModel, metadata: &TransactionMetadata) -> RegisteredTransactionModel {
        let footprint_mock = Arc::new(MockFootprintServiceTrait::new());
        let user_service = Arc::new(UserService::new_with_services(footprint_mock.clone()));
        let ledger_service = Arc::new(LedgerService::new());
        let charge_service = Arc::new(ChargeService::new_with_services(
            user_service.clone(),
            ledger_service.clone(),
            footprint_mock.clone()
        ));
        let rtx = charge_service.clone().register_transaction_only(user, metadata).await.unwrap();
        rtx
    }
}