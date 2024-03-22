use lithic_client::apis::{Error as LithicApiError, Error};
use thiserror;
use crate::error::api_error::ApiError;

impl <T> From<LithicApiError<T>> for ApiError {
    fn from(value: LithicApiError<T>) -> Self {
        match value {
            Error::Reqwest(e) => ApiError::from(e),
            Error::Serde(e) => ApiError::from(e),
            Error::ResponseError(e) => ApiError::from(e),
            Error::Io(e) => ApiError::from(e),
            _ => ApiError::Unexpected(Box::new(value)),
        }
    }
}