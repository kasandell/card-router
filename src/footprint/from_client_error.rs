use footprint::apis::{Error as FootprintApiError, Error};
use thiserror;
use crate::error::api_error::ApiError;

impl <T> From<FootprintApiError<T>> for ApiError {
    fn from(value: FootprintApiError<T>) -> Self {
        match value {
            Error::Reqwest(e) => ApiError::from(e),
            Error::Serde(e) => ApiError::from(e),
            Error::ResponseError(e) => ApiError::from(e),
            Error::Io(e) => ApiError::from(e),
            _ => ApiError::Unexpected(Box::new(value)),
        }
    }
}