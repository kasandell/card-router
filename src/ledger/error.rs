use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum LedgerError {
    #[error("Duplicate transaction")]
    DuplicateTransaction(#[source] Box<dyn std::error::Error>),
    // should add one for registering a transaction without a child
    #[error("Unexpected ledger error")]
    UnexpectedLedgerError(#[source] Box<dyn std::error::Error>)
}

impl From<DataError> for LedgerError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => LedgerError::DuplicateTransaction(e),
            DataError::NotFound(e) => LedgerError::UnexpectedLedgerError(e),
            DataError::Format(e) => LedgerError::UnexpectedLedgerError(e),
            DataError::Unexpected(e) => LedgerError::UnexpectedLedgerError(e),
        }
    }
}