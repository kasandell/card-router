use actix_web::http::StatusCode;
use serde_json::Error as SerdeError;
use crate::error::api_error::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum LithicError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("Unauthorized footprint request")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Conflicting footprint request")]
    Conflict(#[source] Box<dyn std::error::Error>),
    #[error("Footprint request not found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected footprint error")]
    Unexpected(#[source] Box<dyn std::error::Error>)

}


impl From<ApiError> for LithicError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Unauthorized(e) => LithicError::Unauthorized(Box::new(e)),
            ApiError::NotFound(e) => LithicError::NotFound(Box::new(e)),
            ApiError::BadRequest(e) => LithicError::Unexpected(Box::new(e)),
            ApiError::Conflict(e) => LithicError::Conflict(e),
            ApiError::InternalServerError(e) => LithicError::Unexpected(e),
            ApiError::Timeout(e) => LithicError::Unexpected(e),
            ApiError::Unexpected(e) => LithicError::Unexpected(e)
        }
    }
}
