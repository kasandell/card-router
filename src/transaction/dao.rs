use uuid::Uuid;
use crate::data_error::DataError;
use crate::transaction::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait TransactionDaoTrait {
    fn insert_registered_transaction(&self, transaction: InsertableRegisteredTransaction) -> Result<RegisteredTransaction, DataError>;
    fn get_registered_transaction_by_transaction_id(&self, id: Uuid) -> Result<RegisteredTransaction, DataError>;
    fn get_registered_transaction(&self, id: i32) -> Result<RegisteredTransaction, DataError>;
    fn insert_inner_charge(&self, transaction: InsertableInnerChargeLedger) -> Result<InnerChargeLedger, DataError>;
    fn get_inner_charges_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<Vec<InnerChargeLedger>, DataError>;
    fn get_successful_inner_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<InnerChargeLedger, DataError>;
    fn get_inner_charge_by_id(&self, id: i32) -> Result<InnerChargeLedger, DataError>;
    fn insert_outer_charge(&self, transaction: InsertableOuterChargeLedger) -> Result<OuterChargeLedger, DataError>;
    fn get_outer_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<OuterChargeLedger, DataError>;
    fn get_outer_charge_by_id(&self, id: i32) -> Result<OuterChargeLedger, DataError>;
    fn insert_transaction_ledger(&self, transaction: InsertableTransactionLedger) -> Result<TransactionLedger, DataError>;
    fn get_transaction_ledger_by_registered_transaction_id(&self, id: Uuid) -> Result<TransactionLedger, DataError>;
    fn get_transaction_ledger_by_id(&self, id: i32) -> Result<TransactionLedger, DataError>;
}

pub struct TransactionDao {}

impl TransactionDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl TransactionDaoTrait for TransactionDao {
    fn insert_registered_transaction(&self, transaction: InsertableRegisteredTransaction) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::insert(transaction)
    }

    fn get_registered_transaction_by_transaction_id(&self, id: Uuid) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get_by_transaction_id(id)
    }

    fn get_registered_transaction(&self, id: i32) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get(id)
    }

    fn insert_inner_charge(&self, transaction: InsertableInnerChargeLedger) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::insert(transaction)
    }

    fn get_inner_charges_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<Vec<InnerChargeLedger>, DataError> {
        InnerChargeLedger::get_inner_charges_by_registered_transaction(registered_transaction)
    }

    fn get_successful_inner_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::get_successful_inner_charge_by_registered_transaction(registered_transaction)
    }

    fn get_inner_charge_by_id(&self, id: i32) -> Result<InnerChargeLedger, DataError> {
        InnerChargeLedger::get_by_id(id)
    }

    fn insert_outer_charge(&self, transaction: InsertableOuterChargeLedger) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::insert(transaction)
    }

    fn get_outer_charge_by_registered_transaction(&self, registered_transaction: Uuid) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::get_outer_charge_by_registered_transaction(registered_transaction)
    }

    fn get_outer_charge_by_id(&self, id: i32) -> Result<OuterChargeLedger, DataError> {
        OuterChargeLedger::get_by_id(id)
    }

    fn insert_transaction_ledger(&self, transaction: InsertableTransactionLedger) -> Result<TransactionLedger, DataError> {
        TransactionLedger::insert(transaction)
    }

    fn get_transaction_ledger_by_registered_transaction_id(&self, id: Uuid) -> Result<TransactionLedger, DataError> {
        TransactionLedger::get_by_registered_transaction_id(id)
    }

    fn get_transaction_ledger_by_id(&self, id: i32) -> Result<TransactionLedger, DataError> {
        TransactionLedger::get_by_id(id)
    }

}