use std::fmt;
use serde::Deserialize;
use lithic_client::apis::Error as LithicError;
use serde_json::Error as SerdeError;
use crate::error::error::ServiceError;

impl <T> From<LithicError<T>> for ServiceError {
    fn from(error: LithicError<T>) -> ServiceError {
        match &error {
            err => ServiceError::Unexpected(Box::new(error))
        }
    }
}
