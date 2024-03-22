use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::passthrough_card::error::PassthroughCardError;
use crate::wallet::error::WalletError;

#[derive(thiserror::Error, Debug)]
pub enum LithicHandlerError {
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl ResponseError for LithicHandlerError {
    fn status_code(&self) -> StatusCode {
        match self {
            LithicHandlerError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// TODO: probably maperr in code rather than overarching conversion
impl From<PassthroughCardError> for LithicHandlerError {
    fn from(value: PassthroughCardError) -> Self {
        LithicHandlerError::Unexpected(Box::new(value))
    }
}

impl From<WalletError> for LithicHandlerError {
    fn from(value: WalletError) -> Self {
        LithicHandlerError::Unexpected(Box::new(value))
    }
}