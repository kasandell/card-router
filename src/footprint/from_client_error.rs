use std::fmt::Debug;
use footprint::apis::{Error as FootprintApiError, Error};
use thiserror;
use crate::error::api_error::ApiError;

impl <T> From<FootprintApiError<T>> for ApiError {
    fn from(value: FootprintApiError<T>) -> Self {
        match value {
            Error::Reqwest(e) => ApiError::from(e),
            Error::Serde(e) => ApiError::from(e),
            Error::ResponseError(e) => ApiError::from(
                (e.status, e.content.into())
            ),
            Error::Io(e) => ApiError::from(e),
        }
    }
}