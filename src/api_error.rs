use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use adyen_checkout::apis::Error as AdyenCheckoutError;
use adyen_service::checkout::error::Error as AdyenServiceError;
use crate::lithic_service::error::Error as LithicServiceError;
use crate::charge_engine::error::Error as ChargeEngineError;
use crate::transaction::error::{Error as LedgerError, Error};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use r2d2::Error as R2D2Error;
use serde_json::{json, Error as SerdeError};
use std::fmt;
use std::num::ParseIntError;
use crate::adyen_service;
use crate::data_error::DataError;
use crate::service_error::ServiceError;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError { status_code, message }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<SerdeError> for ApiError {
    fn from(error: SerdeError) -> ApiError {
        info!("Converting from serde error");
        println!("SERDE ERROR");
        match error {
            err => ApiError::new(500, format!("Serde Error error: {}", err)),
        }
    }
}



impl From<ChargeEngineError> for ApiError {
    fn from(_: ChargeEngineError) -> Self {
        info!("Converting from charge engine error");
        ApiError::new(500, "Service error".to_string())

    }
}

impl From<LedgerError> for ApiError {
    fn from(_: LedgerError) -> Self {
        info!("Converting from ledger error");
        ApiError::new(500, "Service error".to_string())

    }
}


impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => {
                warn!("{}: {}", self.status_code, self.message);
                self.message.clone()
            },
            false => {
                error!("{}: {}", self.status_code, self.message);
                "Internal server error".to_string()
            },
        };

        HttpResponse::build(status_code)
            .json(json!({ "message": message }))
    }
}

impl From<R2D2Error> for ApiError {
    fn from(_: R2D2Error) -> ApiError {
        ApiError::new(500, "R2D2 error".to_string())
    }
}

impl From<DataError> for ApiError {
    fn from(error: DataError) -> ApiError {
        ApiError::new(error.status_code, error.message)
    }
}

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> ApiError {
        ApiError::new(error.status_code, error.message)
    }
}
