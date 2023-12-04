use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use adyen_checkout::apis::payments_api::PostPaymentsError;
use adyen_checkout::apis::Error as AdyenCheckoutError;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::{json, Error as SerdeError};
use std::fmt;

#[derive(Debug, Deserialize)]
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

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<SerdeError> for ApiError {
    fn from(error: SerdeError) -> ApiError {
        match error {
            err => ApiError::new(500, format!("Serde Error error: {}", err)),
        }
    }
}

impl <T> From<AdyenCheckoutError<T>> for ApiError {
    fn from(error: AdyenCheckoutError<T>) -> ApiError {
        match error {
            err => ApiError::new(500, format!("Adyen error: {}", err)),
        }
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