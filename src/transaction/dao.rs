use uuid::Uuid;
use crate::data_error::DataError;
use crate::transaction::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TransactionDaoTrait {
    async fn insert_registered_transaction(&self, transaction: InsertableRegisteredTransaction) -> Result<RegisteredTransaction, DataError>;
    async fn get_registered_transaction_by_transaction_id(&self, id: Uuid) -> Result<RegisteredTransaction, DataError>;
    async fn get_registered_transaction(&self, id: i32) -> Result<RegisteredTransaction, DataError>;
    async fn insert_inner_charge(&self, transaction: InsertableInnerChargeLedger) -> Result<InnerChargeLedger, DataError>;
    async fn get_inner_charges_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<Vec<InnerChargeLedger>, DataError>;
    async fn get_successful_inner_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<InnerChargeLedger, DataError>;
    async fn get_inner_charge_by_id(&self, id: i32) -> Result<InnerChargeLedger, DataError>;
    async fn insert_outer_charge(&self, transaction: InsertableOuterChargeLedger) -> Result<OuterChargeLedger, DataError>;
    async fn get_outer_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<OuterChargeLedger, DataError>;
    async fn get_outer_charge_by_id(&self, id: i32) -> Result<OuterChargeLedger, DataError>;
    async fn insert_transaction_ledger(&self, transaction: InsertableTransactionLedger) -> Result<TransactionLedger, DataError>;
    async fn get_transaction_ledger_by_registered_transaction_id(&self, id: Uuid) -> Result<TransactionLedger, DataError>;
    async fn get_transaction_ledger_by_id(&self, id: i32) -> Result<TransactionLedger, DataError>;
}

pub struct TransactionDao {}

impl TransactionDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl TransactionDaoTrait for TransactionDao {
    async fn insert_registered_transaction(&self, transaction: InsertableRegisteredTransaction) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::insert(transaction).await
    }

    async fn get_registered_transaction_by_transaction_id(&self, id: Uuid) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get_by_transaction_id(id).await
    }

    async fn get_registered_transaction(&self, id: i32) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get(id).await
    }

    async fn insert_inner_charge(&self, transaction: InsertableInnerChargeLedger) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::insert(transaction).await
    }

    async fn get_inner_charges_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<Vec<InnerChargeLedger>, DataError> {
        InnerChargeLedger::get_inner_charges_by_registered_transaction(registered_transaction).await
    }

    async fn get_successful_inner_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::get_successful_inner_charge_by_registered_transaction(registered_transaction).await
    }

    async fn get_inner_charge_by_id(&self, id: i32) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::get_by_id(id).await
    }

    async fn insert_outer_charge(&self, transaction: InsertableOuterChargeLedger) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::insert(transaction).await
    }

    async fn get_outer_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::get_outer_charge_by_registered_transaction(registered_transaction).await
    }

    async fn get_outer_charge_by_id(&self, id: i32) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::get_by_id(id).await
    }

    async fn insert_transaction_ledger(&self, transaction: InsertableTransactionLedger) -> Result<TransactionLedger, DataError> {
        TransactionLedger::insert(transaction).await
    }

    async fn get_transaction_ledger_by_registered_transaction_id(&self, id: Uuid) -> Result<TransactionLedger, DataError> {
        TransactionLedger::get_by_registered_transaction_id(id).await
    }

    async fn get_transaction_ledger_by_id(&self, id: i32) -> Result<TransactionLedger, DataError> {
        TransactionLedger::get_by_id(id).await
    }

}