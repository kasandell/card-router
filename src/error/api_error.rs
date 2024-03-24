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
        _ => ApiError::Unexpected(error)
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

impl From<(StatusCode, Box<dyn std::error::Error>)> for ApiError {
    fn from(value: (StatusCode, Box<dyn StdErr>)) -> Self {
        let code = value.0;
        let error = value.1;
        api_error_for_status_code(code, error)
    }
}

#[cfg(test)]
impl PartialEq for ApiError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ApiError::Unexpected(_), ApiError::Unexpected(_))
            | (ApiError::BadRequest(_), ApiError::BadRequest(_))
            | (ApiError::Conflict(_), ApiError::Conflict(_))
            | (ApiError::InternalServerError(_), ApiError::InternalServerError(_))
            | (ApiError::Timeout(_), ApiError::Timeout(_))
            | (ApiError::NotFound(_), ApiError::NotFound(_))
            | (ApiError::Unauthorized(_), ApiError::Unauthorized(_))
            => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use reqwest::StatusCode;
    use crate::error::api_error::{api_error_for_status_code, ApiError};
    use crate::test_helper::error::serde_error;

    #[test]
    pub fn test_from_serde() {
        let base_error = "test";
        assert_eq!(ApiError::Unexpected(base_error.into()), ApiError::from(serde_error()));
    }

    #[test]
    pub fn test_from_status_code() {
        let base_error = "test";
        assert_eq!(ApiError::NotFound(base_error.into()), api_error_for_status_code(StatusCode::NOT_FOUND, base_error.into()));
        assert_eq!(ApiError::BadRequest(base_error.into()), api_error_for_status_code(StatusCode::BAD_REQUEST, base_error.into()));
        assert_eq!(ApiError::Unauthorized(base_error.into()), api_error_for_status_code(StatusCode::UNAUTHORIZED, base_error.into()));
        assert_eq!(ApiError::Unexpected(base_error.into()), api_error_for_status_code(StatusCode::REQUEST_TIMEOUT, base_error.into()));
        assert_eq!(ApiError::Unexpected(base_error.into()), api_error_for_status_code(StatusCode::GATEWAY_TIMEOUT, base_error.into()));
        assert_eq!(ApiError::Unexpected(base_error.into()), api_error_for_status_code(StatusCode::INTERNAL_SERVER_ERROR, base_error.into()));
    }
}