use thiserror;
use crate::ledger::error::LedgerError;

#[derive(thiserror::Error, Debug)]
pub enum ChargeError {
    #[error("No card present to charge in the request")]
    NoCardInRequest,
    #[error("Unexpected charge error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl From<LedgerError> for ChargeError {
    fn from(value: LedgerError) -> Self {
        ChargeError::Unexpected(Box::new(value))
    }
}


