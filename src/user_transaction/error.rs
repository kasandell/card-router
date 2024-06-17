use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum UserTransactionError {
    #[error("Not found")]
    NotFound(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Not found")]
    Unauthorized(#[source] Box<dyn std::error::Error + Send + Sync>),
    // should add one for registering a transaction without a child
    #[error("Unexpected transaction error")]
    UnexpectedError(#[source] Box<dyn std::error::Error + Send + Sync>)
}

impl ResponseError for UserTransactionError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserTransactionError::NotFound(_) => StatusCode::NOT_FOUND,
            UserTransactionError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            UserTransactionError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DataError> for UserTransactionError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => UserTransactionError::UnexpectedError(e),
            DataError::NotFound(e) => UserTransactionError::NotFound(e),
            DataError::Format(e) => UserTransactionError::UnexpectedError(e),
            DataError::Unexpected(e) => UserTransactionError::UnexpectedError(e),
        }
    }
}

#[cfg(test)]
impl PartialEq for UserTransactionError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UserTransactionError::NotFound(_), UserTransactionError::NotFound(_))
            | (UserTransactionError::Unauthorized(_), UserTransactionError::Unauthorized(_))
            | (UserTransactionError::UnexpectedError(_), UserTransactionError::UnexpectedError(_)) => true,
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
        assert_eq!(LedgerError::Unexpected(BASE_ERROR.into()), LedgerError::from(DataError::Format(BASE_ERROR.into())));
        assert_eq!(LedgerError::Unexpected(BASE_ERROR.into()), LedgerError::from(DataError::NotFound(BASE_ERROR.into())));
        assert_eq!(LedgerError::Unexpected(BASE_ERROR.into()), LedgerError::from(DataError::Unexpected(BASE_ERROR.into())));
    }
}
