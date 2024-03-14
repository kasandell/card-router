use std::fmt;
use crate::error::data_error::DataError;
use adyen_checkout::apis::Error as AdyenCheckoutError;
use crate::adyen::checkout::error::Error as AdyenCheckoutServiceError;
use crate::lithic::error::Error as LithicServiceError;
use serde_json::{json, Error as SerdeError};
use crate::error::error_type::ErrorType;
use footprint::apis::Error as FootprintError;
use crate::footprint::service::FootprintService;

#[derive(Debug, Clone)]
pub struct ServiceError {
    pub error_type: ErrorType,
    pub message: String
}

impl ServiceError {
    pub fn new(error_type: ErrorType, message: &str) -> ServiceError {
        ServiceError { error_type, message: message.to_string() }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DataError> for ServiceError {
    fn from(error: DataError) -> ServiceError {
        ServiceError::new(error.error_type, &error.message)
    }
}

impl <T> From<AdyenCheckoutError<T>> for ServiceError {
    fn from(error: AdyenCheckoutError<T>) -> ServiceError {
        tracing::info!("Converting from adyen checkout error");
        tracing::info!("Converting from adyen checkout error");
        tracing::info!("{}", error);
        match error {
            err => ServiceError::new(ErrorType::InternalServerError, &format!("Adyen error")),
        }
    }
}

impl From<AdyenCheckoutServiceError> for ServiceError {
    fn from(error: AdyenCheckoutServiceError) -> Self {
        tracing::info!("Converting from adyen service error");
        tracing::info!("Converting from adyen service error");
        tracing::info!("{:?}", error);
        ServiceError::new(ErrorType::InternalServerError, "Service error")
    }
}

impl <T> From<FootprintError<T>> for ServiceError {
    fn from(error: FootprintError<T>) -> Self {
        tracing::info!("Converting from footprint error");
        tracing::info!("Converting from footprint error");
        tracing::info!("{}", error.to_string());
        ServiceError::new(ErrorType::InternalServerError, "Service error")
    }
}


impl From<LithicServiceError> for ServiceError {
    fn from(error: LithicServiceError) -> Self {
        tracing::info!("converting from lithic service error");
        tracing::info!("{:?}", error);
        ServiceError::new(ErrorType::InternalServerError, "Lithic service error")
    }
}

impl From<SerdeError> for ServiceError {
    fn from(error: SerdeError) -> ServiceError {
        tracing::info!("Converting from serde error");
        tracing::info!("{}", error.to_string());
        match error {
            err => ServiceError::new(ErrorType::InternalServerError, &format!("Serde Error error: {}", err)),
        }
    }
}
