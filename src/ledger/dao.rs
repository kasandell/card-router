use std::sync::Arc;
use uuid::Uuid;
use crate::data_error::DataError;
use crate::ledger::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LedgerDaoTrait {
    async fn insert_registered_transaction<'a>(self: Arc<Self>, transaction: &InsertableRegisteredTransaction<'a>) -> Result<RegisteredTransaction, DataError>;
    async fn get_registered_transaction_by_transaction_id(self: Arc<Self>, id: &Uuid) -> Result<RegisteredTransaction, DataError>;
    async fn get_registered_transaction(self: Arc<Self>, id: i32) -> Result<RegisteredTransaction, DataError>;
    async fn insert_inner_charge(self: Arc<Self>, transaction: &InsertableInnerChargeLedger) -> Result<InnerChargeLedger, DataError>;
    async fn get_inner_charges_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<Vec<InnerChargeLedger>, DataError>;
    async fn get_successful_inner_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<InnerChargeLedger, DataError>;
    async fn get_inner_charge_by_id(self: Arc<Self>, id: i32) -> Result<InnerChargeLedger, DataError>;
    async fn insert_outer_charge(self: Arc<Self>, transaction: &InsertableOuterChargeLedger) -> Result<OuterChargeLedger, DataError>;
    async fn get_outer_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<OuterChargeLedger, DataError>;
    async fn get_outer_charge_by_id(self: Arc<Self>, id: i32) -> Result<OuterChargeLedger, DataError>;
    async fn insert_transaction_ledger(self: Arc<Self>, transaction: &InsertableTransactionLedger) -> Result<TransactionLedger, DataError>;
    async fn get_transaction_ledger_by_registered_transaction_id(self: Arc<Self>, id: i32) -> Result<TransactionLedger, DataError>;
    async fn get_transaction_ledger_by_id(self: Arc<Self>, id: i32) -> Result<TransactionLedger, DataError>;
}

pub struct LedgerDao {}

impl LedgerDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl LedgerDaoTrait for LedgerDao {
    async fn insert_registered_transaction<'a>(self: Arc<Self>, transaction: &InsertableRegisteredTransaction<'a>) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::insert(transaction).await
    }

    async fn get_registered_transaction_by_transaction_id(self: Arc<Self>, id: &Uuid) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get_by_transaction_id(id).await
    }

    async fn get_registered_transaction(self: Arc<Self>, id: i32) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get(id).await
    }

    async fn insert_inner_charge(self: Arc<Self>, transaction: &InsertableInnerChargeLedger) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::insert(transaction).await
    }

    async fn get_inner_charges_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<Vec<InnerChargeLedger>, DataError> {
        InnerChargeLedger::get_inner_charges_by_registered_transaction(registered_transaction).await
    }

    async fn get_successful_inner_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::get_successful_inner_charge_by_registered_transaction(registered_transaction).await
    }

    async fn get_inner_charge_by_id(self: Arc<Self>, id: i32) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::get_by_id(id).await
    }

    async fn insert_outer_charge(self: Arc<Self>, transaction: &InsertableOuterChargeLedger) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::insert(transaction).await
    }

    async fn get_outer_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::get_outer_charge_by_registered_transaction(registered_transaction).await
    }

    async fn get_outer_charge_by_id(self: Arc<Self>, id: i32) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::get_by_id(id).await
    }

    async fn insert_transaction_ledger(self: Arc<Self>, transaction: &InsertableTransactionLedger) -> Result<TransactionLedger, DataError> {
        TransactionLedger::insert(transaction).await
    }

    async fn get_transaction_ledger_by_registered_transaction_id(self: Arc<Self>, id: i32) -> Result<TransactionLedger, DataError> {
        TransactionLedger::get_by_registered_transaction_id(id).await
    }

    async fn get_transaction_ledger_by_id(self: Arc<Self>, id: i32) -> Result<TransactionLedger, DataError> {
        TransactionLedger::get_by_id(id).await
    }

}