use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum WalletError {
    #[error("User is not the owner of specified card")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Wallet not found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Card already matched")]
    Conflict(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl ResponseError for WalletError {
    fn status_code(&self) -> StatusCode {
        match self {
            WalletError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            WalletError::NotFound(_) => StatusCode::NOT_FOUND,
            WalletError::Conflict(_) => StatusCode::CONFLICT,
            WalletError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DataError> for WalletError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => WalletError::Conflict(Box::new(e)),
            DataError::NotFound(e) => WalletError::NotFound(Box::new(e)),
            DataError::Format(e) => WalletError::Unexpected(Box::new(e)),
            DataError::Unexpected(e) => WalletError::Unexpected(Box::new(e))
        }
    }
}