use actix_web::{Responder, ResponseError};
use actix_web::http::StatusCode;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("Unauthorized User")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Not found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected user error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            UserError::NotFound(_) => StatusCode::NOT_FOUND,
            UserError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<DataError> for UserError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => UserError::Unexpected(e),
            DataError::NotFound(e) => UserError::NotFound(e),
            DataError::Format(e) => UserError::Unexpected(e),
            DataError::Unexpected(e) => UserError::Unexpected(e)
        }
    }
}