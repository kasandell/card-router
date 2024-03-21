use adyen_checkout::apis::{Error as AdyenCheckoutError, Error};
use thiserror;
use crate::error::api_error::ApiError;

impl <T> From<AdyenCheckoutError<T>> for ApiError {
    fn from(value: AdyenCheckoutError<T>) -> Self {
        match value {
            Error::Reqwest(e) => ApiError::from(e),
            Error::Serde(e) => ApiError::from(e),
            Error::ResponseError(e) => ApiError::from(e),
            Error::Io(e) => ApiError::from(e),
            _ => ApiError::Unexpected(Box::new(value)),
        }
    }
}