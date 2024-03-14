use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use adyen_checkout::apis::Error as AdyenCheckoutError;
use adyen::checkout::error::Error as AdyenServiceError;
use crate::lithic::error::Error as LithicServiceError;
use crate::charge::error::Error as ChargeEngineError;
use crate::ledger::error::{Error as LedgerError, Error};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use r2d2::Error as R2D2Error;
use serde_json::{json, Error as SerdeError};
use std::fmt;
use std::num::ParseIntError;
use crate::adyen;
use crate::error::data_error::DataError;
use crate::error::error_type::ErrorType;
use crate::error::service_error::ServiceError;

#[derive(Debug, Clone)]
pub struct ApiError {
    pub error_type: ErrorType,
    pub message: String,
}

impl ApiError {
    pub fn new(error_type: ErrorType, message: &str) -> ApiError {
        ApiError { error_type, message: message.to_string() }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<SerdeError> for ApiError {
    fn from(error: SerdeError) -> ApiError {
        tracing::info!("Converting from serde error");
        tracing::info!("SERDE ERROR");
        match error {
            err => ApiError::new(ErrorType::InternalServerError, &format!("Serde Error error: {}", err)),
        }
    }
}



impl From<ChargeEngineError> for ApiError {
    fn from(_: ChargeEngineError) -> Self {
        tracing::info!("Converting from charge engine error");
        ApiError::new(ErrorType::InternalServerError, "Service error")

    }
}

impl From<LedgerError> for ApiError {
    fn from(_: LedgerError) -> Self {
        tracing::info!("Converting from ledger error");
        ApiError::new(ErrorType::InternalServerError, "Service error")

    }
}


impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self.error_type {

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => {
                tracing::warn!("{}: {}", self.error_type, self.message);
                self.message.clone()
            },
            false => {
                tracing::error!("{}: {}", self.error_type, self.message);
                "Internal server error".to_string()
            },
        };

        HttpResponse::build(status_code)
            .json(json!({ "message": message }))
    }
}

impl From<R2D2Error> for ApiError {
    fn from(_: R2D2Error) -> ApiError {
        ApiError::new(ErrorType::InternalServerError, "R2D2 error")
    }
}

impl From<DataError> for ApiError {
    fn from(error: DataError) -> ApiError {
        ApiError::new(error.error_type, &error.message)
    }
}

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> ApiError {
        ApiError::new(error.error_type, &error.message)
    }
}
