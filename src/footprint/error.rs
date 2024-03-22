use actix_web::http::StatusCode;
use serde_json::Error as SerdeError;
use crate::error::api_error::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum FootprintError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("Unauthorized footprint request")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Bad request")]
    BadRequest(#[source] Box<dyn std::error::Error>),
    #[error("Conflicting footprint request")]
    Conflict(#[source] Box<dyn std::error::Error>),
    #[error("Footprint request not found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected footprint error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}


impl From<ApiError> for FootprintError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Unauthorized(e) => FootprintError::Unauthorized(e),
            ApiError::NotFound(e) => FootprintError::NotFound(e),
            ApiError::BadRequest(e) => FootprintError::Unexpected(e),
            ApiError::Conflict(e) => FootprintError::Conflict(e),
            ApiError::InternalServerError(e) => FootprintError::Unexpected(e),
            ApiError::Timeout(e) => FootprintError::Unexpected(e),
            ApiError::Unexpected(e) => FootprintError::Unexpected(e)
        }
    }
}


impl From<SerdeError> for FootprintError {
    fn from(value: SerdeError) -> Self {
        FootprintError::BadRequest("Formatting error".into())
    }
}