use std::str::{FromStr, Utf8Error};
use std::string::FromUtf8Error;
use base64::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum PaginationError {
    #[error("Bad Request")]
    BadRequest(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unexpected pagination error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)
}


impl From<base64::DecodeError> for PaginationError {
    fn from(value: DecodeError) -> Self {
        PaginationError::Unexpected(value.into())
    }
}

impl From<Utf8Error> for PaginationError {
    fn from(value: Utf8Error) -> Self {
        PaginationError::Unexpected(value.into())
    }
}

impl From<FromUtf8Error> for PaginationError {
    fn from(value: FromUtf8Error) -> Self {
        PaginationError::Unexpected(value.into())
    }
}