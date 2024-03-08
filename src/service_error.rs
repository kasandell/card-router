use std::fmt;
use crate::data_error::DataError;
use adyen_checkout::apis::Error as AdyenCheckoutError;
use crate::adyen_service::checkout::error::Error as AdyenCheckoutServiceError;
use crate::lithic_service::error::Error as LithicServiceError;
use serde_json::{json, Error as SerdeError};
use crate::error_type::ErrorType;

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
        info!("Converting from adyen checkout error");
        println!("Converting from adyen checkout error");
        println!("{}", error);
        match error {
            err => ServiceError::new(ErrorType::InternalServerError, &format!("Adyen error")),
        }
    }
}

impl From<AdyenCheckoutServiceError> for ServiceError {
    fn from(_: AdyenCheckoutServiceError) -> Self {
        info!("Converting from adyen service error");
        println!("Converting from adyen service error");
        ServiceError::new(ErrorType::InternalServerError, "Service error")

    }
}


impl From<LithicServiceError> for ServiceError {
    fn from(_: LithicServiceError) -> Self {
        info!("converting from lithic service error");
        ServiceError::new(ErrorType::InternalServerError, "Lithic service error")
    }
}

impl From<SerdeError> for ServiceError {
    fn from(error: SerdeError) -> ServiceError {
        info!("Converting from serde error");
        match error {
            err => ServiceError::new(ErrorType::InternalServerError, &format!("Serde Error error: {}", err)),
        }
    }
}
