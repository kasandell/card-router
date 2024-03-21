use std::fmt::Debug;
use actix_web::http::StatusCode;
use serde_json::Error as SerdeError;
use std::error::Error as StdErr;
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized(#[source] Box<dyn std::error::Error>),

    #[error("Not found")]
    NotFound(#[source] Box<dyn std::error::Error>),

    #[error("BadRequest")]
    BadRequest(#[source] Box<dyn std::error::Error>),

    #[error("Conflict")]
    Conflict(#[source] Box<dyn std::error::Error>),

    #[error("Internal Server error")]
    InternalServerError(#[source] Box<dyn std::error::Error>),


    #[error("Timeout")]
    Timeout(#[source] Box<dyn std::error::Error>),

    #[error("Unexpected API Error")]
    Unexpected(#[source] Box<dyn std::error::Error>),
}

fn api_error_for_status_code(status: StatusCode, error: Box<dyn std::error::Error>) -> ApiError {
    match status {
        StatusCode::CONFLICT => ApiError::Conflict(error),
        StatusCode::BAD_REQUEST => ApiError::BadRequest(error),
        StatusCode::NOT_FOUND => ApiError::NotFound(error),
        StatusCode::UNAUTHORIZED => ApiError::Unauthorized(error),
        StatusCode(_) => ApiError::Unexpected(error)
    }
}


impl From<SerdeError> for ApiError {
    fn from(error: SerdeError) -> ApiError {
        match &error {
            err => ApiError::Unexpected(Box::new(error))
        }
    }
}


impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> ApiError {
        if error.is_status() {
            match error.status() {
                Some(status) => {
                    api_error_for_status_code(status, Box::new(error))
                }
                None => ApiError::Unexpected(Box::new(error))
            }
        } else if error.is_timeout() {
            ApiError::Timeout(Box::new(error))
        } else {
            ApiError::Unexpected(Box::new(error))
        }
    }
}


impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> ApiError {
        ApiError::Unexpected(Box::new(error))
    }
}


impl From<(Option<i32>, dyn StdErr)> for ApiError {
    fn from(value: Box<(Option<i32>, dyn StdErr)>) -> Self {
        match value.0 {
            Some(code) => {
                let cast = u16::try_from(code);
                match cast {
                    Ok(code) => {
                        match StatusCode::from_u16(code) {
                            Ok(status_code) => {
                                api_error_for_status_code(status_code, Box::new(value.1))
                            }
                            Err(_) => {
                                ApiError::Unexpected(Box::new(value.1))
                            }
                        }
                    },
                    Err(_) => {
                        ApiError::Unexpected(Box::new(value.1))
                    }
                }
            },
            None => ApiError::Unexpected(Box::new(value.1))
        }
    }
}


