use std::fmt;
use serde::Deserialize;
use crate::data_error::DataError;
use adyen_checkout::apis::Error as AdyenCheckoutError;
use crate::adyen_service::checkout::error::Error as AdyenCheckoutServiceError;
use crate::lithic_service::error::Error as LithicServiceError;
use serde_json::{json, Error as SerdeError};
use crate::charge_engine::error::Error as ChargeEngineError;

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceError {
    pub status_code: u16,
    pub message: String
}

impl ServiceError {
    pub fn new(status_code: u16, message: String) -> ServiceError {
        ServiceError { status_code, message }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DataError> for ServiceError {
    fn from(error: DataError) -> ServiceError {
        ServiceError::new(error.status_code, error.message)
    }
}

impl <T> From<AdyenCheckoutError<T>> for ServiceError {
    fn from(error: AdyenCheckoutError<T>) -> ServiceError {
        info!("Converting from adyen checkout error");
        println!("Converting from adyen checkout error");
        println!("{}", error);
        match error {
            err => ServiceError::new(500, format!("Adyen error")),
        }
    }
}

impl From<AdyenCheckoutServiceError> for ServiceError {
    fn from(_: AdyenCheckoutServiceError) -> Self {
        info!("Converting from adyen service error");
        println!("Converting from adyen service error");
        ServiceError::new(500, "Service error".to_string())

    }
}


impl From<LithicServiceError> for ServiceError {
    fn from(_: LithicServiceError) -> Self {
        info!("converting from lithic service error");
        ServiceError::new(500, "Lithic service error".to_string())
    }
}

impl From<SerdeError> for ServiceError {
    fn from(error: SerdeError) -> ServiceError {
        info!("Converting from serde error");
        match error {
            err => ServiceError::new(500, format!("Serde Error error: {}", err)),
        }
    }
}
