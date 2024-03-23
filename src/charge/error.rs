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



#[cfg(test)]
mod test {
    use crate::charge::error::ChargeError;
    use crate::ledger::error::LedgerError;

    #[test]
    pub fn test_from_ledger_error() {
        let base_error = "test";
        /*
        assert_eq!(ChargeError::Unexpected(base_error.clone().into()), ChargeError::from(LedgerError::UnexpectedLedgerError(base_error.clone().into())));
        assert_eq!(ChargeError::Unexpected(base_error.clone().into()), ChargeError::from(LedgerError::DuplicateTransaction(base_error.clone().into())));

         */
    }

}

