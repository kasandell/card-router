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

#[cfg(test)]
impl PartialEq for LedgerError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LedgerError::DuplicateTransaction(_), LedgerError::DuplicateTransaction(_))
            | (LedgerError::UnexpectedLedgerError(_), LedgerError::UnexpectedLedgerError(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::data_error::DataError;
    use crate::ledger::error::LedgerError;

    const BASE_ERROR: &str = "test";
    #[test]
    pub fn from_data_error() {
        assert_eq!(LedgerError::DuplicateTransaction(BASE_ERROR.into()), LedgerError::from(DataError::Conflict(BASE_ERROR.into())));
        assert_eq!(LedgerError::UnexpectedLedgerError(BASE_ERROR.into()), LedgerError::from(DataError::Format(BASE_ERROR.into())));
        assert_eq!(LedgerError::UnexpectedLedgerError(BASE_ERROR.into()), LedgerError::from(DataError::NotFound(BASE_ERROR.into())));
        assert_eq!(LedgerError::UnexpectedLedgerError(BASE_ERROR.into()), LedgerError::from(DataError::Unexpected(BASE_ERROR.into())));
    }
}
