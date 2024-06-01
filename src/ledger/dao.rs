use std::sync::Arc;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::error::data_error::DataError;
use crate::ledger::entity::{
    PendingWalletTransactionLedger,
    InsertablePendingWalletTransactionLedger,

    SettledWalletTransactionLedger,
    InsertableSettledWalletTransactionLedger,

    PendingPassthroughCardTransactionLedger,
    InsertablePendingPassthroughCardTransactionLedger,

    SettledPassthroughCardTransactionLedger,
    InsertableSettledPassthroughCardTransactionLedger
};
use crate::passthrough_card::model::PassthroughCardModel;
use crate::util::transaction::Transaction;
use crate::wallet::model::WalletModel;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait LedgerDaoTrait {
    async fn insert_settled_wallet_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertableSettledWalletTransactionLedger) -> Result<SettledWalletTransactionLedger, DataError>;
    async fn insert_pending_wallet_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertablePendingWalletTransactionLedger) -> Result<PendingWalletTransactionLedger, DataError>;
    async fn insert_settled_passthrough_card_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertableSettledPassthroughCardTransactionLedger) -> Result<SettledPassthroughCardTransactionLedger, DataError>;
    async fn insert_pending_passthrough_card_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertablePendingPassthroughCardTransactionLedger) -> Result<PendingPassthroughCardTransactionLedger, DataError>;
}

pub struct LedgerDao {}

impl LedgerDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl LedgerDaoTrait for LedgerDao {
    async fn insert_settled_wallet_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertableSettledWalletTransactionLedger) -> Result<SettledWalletTransactionLedger, DataError> {
        SettledWalletTransactionLedger::insert(transaction, record).await
    }

    async fn insert_pending_wallet_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertablePendingWalletTransactionLedger) -> Result<PendingWalletTransactionLedger, DataError> {
        PendingWalletTransactionLedger::insert(transaction, record).await
    }

    async fn insert_settled_passthrough_card_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertableSettledPassthroughCardTransactionLedger) -> Result<SettledPassthroughCardTransactionLedger, DataError> {
        SettledPassthroughCardTransactionLedger::insert(transaction, record).await
    }

    async fn insert_pending_passthrough_card_transaction<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, record: &InsertablePendingPassthroughCardTransactionLedger) -> Result<PendingPassthroughCardTransactionLedger, DataError> {
        PendingPassthroughCardTransactionLedger::insert(transaction, record).await
    }
}