use std::fmt;
use actix_web::http::StatusCode;
use serde::Deserialize;
use adyen_checkout::apis::{Error as AdyenCheckoutError, Error, ResponseContent};
use serde_json::Error as SerdeError;
use thiserror;
use crate::error::api_error::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum CheckoutError<'a> {
    #[error("Unauthorized checkout attempt")]
    UnauthorizedCheckoutAttempt(#[source] Box<dyn std::error::Error>),
    #[error("Duplicate checkout attempts")]
    DuplicateCheckoutError(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected checkout error")]
    UnexpectedCheckoutError(#[source] Box<dyn std::error::Error>)
}

impl From<ApiError> for CheckoutError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Unauthorized(e) => CheckoutError::UnauthorizedCheckoutAttempt(e),
            ApiError::NotFound(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::BadRequest(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::Conflict(e) => CheckoutError::DuplicateCheckoutError(e),
            ApiError::InternalServerError(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::Timeout(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::Unexpected(e) => CheckoutError::UnexpectedCheckoutError(e),
        }
    }
}