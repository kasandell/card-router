use std::sync::Arc;
use async_trait::async_trait;
use crate::charge::model::RegisteredTransactionModel;
use crate::ledger::constant::{MoneyMovementDirection, MoneyMovementType};
use crate::ledger::dao::{LedgerDao, LedgerDaoTrait};
use crate::ledger::entity::{InsertablePendingPassthroughCardTransactionLedger, InsertablePendingWalletTransactionLedger, InsertableSettledPassthroughCardTransactionLedger, InsertableSettledWalletTransactionLedger};
use crate::ledger::error::LedgerError;
use crate::ledger::model::{PendingPassthroughCardTransactionLedgerModel, PendingWalletTransactionLedgerModel, SettledPassthroughCardTransactionLedgerModel, SettledWalletTransactionLedgerModel};
use crate::passthrough_card::model::PassthroughCardModel;
use crate::util::transaction::Transaction;
use crate::wallet::model::WalletModel;

#[async_trait]
pub trait LedgerServiceTrait {
    async fn reserve_passthrough_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card: &PassthroughCardModel,
        amount_cents: i32
    ) -> Result<PendingPassthroughCardTransactionLedgerModel, LedgerError>;


    async fn release_passthrough_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card: &PassthroughCardModel,
        amount_cents: i32
    ) -> Result<PendingPassthroughCardTransactionLedgerModel, LedgerError>;


    async fn settle_passthrough_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card: &PassthroughCardModel,
        amount_cents: i32
    ) -> Result<SettledPassthroughCardTransactionLedgerModel, LedgerError>;

    async fn reserve_wallet_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card_id: i32,
        amount_cents: i32
    ) -> Result<PendingWalletTransactionLedgerModel, LedgerError>;

    async fn release_wallet_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card_id: i32,
        amount_cents: i32
    ) -> Result<PendingWalletTransactionLedgerModel, LedgerError>;

    async fn settle_wallet_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card_id: i32,
        amount_cents: i32
    ) -> Result<SettledWalletTransactionLedgerModel, LedgerError>;
}

pub struct LedgerService {
    dao: Arc<dyn LedgerDaoTrait + Send + Sync>
}

impl LedgerService {
    pub fn new() -> Self {
        Self {
            dao: Arc::new(LedgerDao {})
        }
    }
}

#[async_trait]
impl LedgerServiceTrait for LedgerService {
    async fn reserve_passthrough_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card: &PassthroughCardModel,
        amount_cents: i32
    ) -> Result<PendingPassthroughCardTransactionLedgerModel, LedgerError> {
        let record = self.dao.clone().insert_pending_passthrough_card_transaction(
            database_transaction,
            &InsertablePendingPassthroughCardTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: card.user_id,
                passthrough_card_id: card.id,
                money_movement_direction: MoneyMovementDirection::Debit,
                money_movement_type: MoneyMovementType::PassthroughCardReserve,
                amount_cents,
            }
        ).await?;
        Ok(record.into())
    }


    async fn release_passthrough_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card: &PassthroughCardModel,
        amount_cents: i32
    ) -> Result<PendingPassthroughCardTransactionLedgerModel, LedgerError> {
        let record = self.dao.clone().insert_pending_passthrough_card_transaction(
            database_transaction,
            &InsertablePendingPassthroughCardTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: card.user_id,
                passthrough_card_id: card.id,
                money_movement_direction: MoneyMovementDirection::Credit,
                money_movement_type: MoneyMovementType::PassthroughCardRelease,
                amount_cents,
            }
        ).await?;
        Ok(record.into())
    }


    async fn settle_passthrough_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card: &PassthroughCardModel,
        amount_cents: i32
    ) -> Result<SettledPassthroughCardTransactionLedgerModel, LedgerError> {
        let pending_record = self.dao.clone().insert_pending_passthrough_card_transaction(
            database_transaction,
            &InsertablePendingPassthroughCardTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: card.user_id,
                passthrough_card_id: card.id,
                money_movement_direction: MoneyMovementDirection::Credit,
                money_movement_type: MoneyMovementType::PassthroughCardSettle,
                amount_cents,
            }
        ).await?;

        let settlement_record = self.dao.clone().insert_settled_passthrough_card_transaction(
            database_transaction,
            &InsertableSettledPassthroughCardTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: card.user_id,
                passthrough_card_id: card.id,
                money_movement_direction: MoneyMovementDirection::Debit,
                money_movement_type: MoneyMovementType::PassthroughCardSettle,
                amount_cents,
            }
        ).await?;
        Ok(settlement_record.into())
    }

    async fn reserve_wallet_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card_id: i32,
        amount_cents: i32
    ) -> Result<PendingWalletTransactionLedgerModel, LedgerError> {
        let record = self.dao.clone().insert_pending_wallet_transaction(
            database_transaction,
            &InsertablePendingWalletTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: registered_transaction.user_id,
                wallet_id: card_id,
                money_movement_direction: MoneyMovementDirection::Credit,
                money_movement_type: MoneyMovementType::WalletReserve,
                amount_cents,
            }
        ).await?;
        Ok(record.into())
    }

    async fn release_wallet_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card_id: i32,
        amount_cents: i32
    ) -> Result<PendingWalletTransactionLedgerModel, LedgerError> {
        let record = self.dao.clone().insert_pending_wallet_transaction(
            database_transaction,
            &InsertablePendingWalletTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: registered_transaction.user_id,
                wallet_id: card_id,
                money_movement_direction: MoneyMovementDirection::Debit,
                money_movement_type: MoneyMovementType::WalletRelease,
                amount_cents,
            }
        ).await?;
        Ok(record.into())
    }

    async fn settle_wallet_card_amount<'a>(
        self: Arc<Self>,
        database_transaction: &mut Transaction<'_, '_>,
        registered_transaction: &RegisteredTransactionModel,
        card_id: i32,
        amount_cents: i32
    ) -> Result<SettledWalletTransactionLedgerModel, LedgerError> {
        let pending_record = self.dao.clone().insert_pending_wallet_transaction(
            database_transaction,
            &InsertablePendingWalletTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: registered_transaction.user_id,
                wallet_id: card_id,
                money_movement_direction: MoneyMovementDirection::Debit,
                money_movement_type: MoneyMovementType::WalletSettle,
                amount_cents,
            }
        ).await?;
        let settled_record = self.dao.clone().insert_settled_wallet_transaction(
            database_transaction,
            &InsertableSettledWalletTransactionLedger {
                registered_transaction_id: registered_transaction.id,
                user_id: registered_transaction.user_id,
                wallet_id: card_id,
                money_movement_direction: MoneyMovementDirection::Credit,
                money_movement_type: MoneyMovementType::WalletSettle,
                amount_cents,
            }
        ).await?;
        Ok(settled_record.into())
    }
}