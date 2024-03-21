use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum CreditCardTypeError {
   #[error("Unexpected Error")]
   Unexpected(#[source] Box<dyn std::error::Error>)
}

impl From<DataError> for CreditCardTypeError {
    fn from(value: DataError) -> Self {
        CreditCardTypeError::Unexpected(Box::new(value))
    }
}

impl ResponseError for CreditCardTypeError {
    fn status_code(&self) -> StatusCode {
        match self {
            CreditCardTypeError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}