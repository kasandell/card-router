use actix_web::http::StatusCode;
use serde_json::Error as SerdeError;
use crate::error::api_error::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum LithicError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("Unauthorized footprint request")]
    Unauthorized(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Conflicting footprint request")]
    Conflict(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Footprint request not found")]
    NotFound(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unexpected footprint error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)

}


impl From<ApiError> for LithicError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Unauthorized(e) => LithicError::Unauthorized(e),
            ApiError::NotFound(e) => LithicError::NotFound(e),
            ApiError::BadRequest(e) => LithicError::Unexpected(e),
            ApiError::Conflict(e) => LithicError::Conflict(e),
            ApiError::InternalServerError(e) => LithicError::Unexpected(e),
            ApiError::Timeout(e) => LithicError::Unexpected(e),
            ApiError::Unexpected(e) => LithicError::Unexpected(e)
        }
    }
}


#[cfg(test)]
impl PartialEq for LithicError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LithicError::NotFound(_), LithicError::NotFound(_))
            | (LithicError::Conflict(_), LithicError::Conflict(_))
            | (LithicError::Unauthorized(_), LithicError::Unauthorized(_))
            | (LithicError::Unexpected(_), LithicError::Unexpected(_))
            | (LithicError::NotImplemented, LithicError::NotImplemented) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::api_error::ApiError;
    use crate::lithic::error::LithicError;

    const BASE_ERROR: &str = "test";

    #[test]
    pub fn from_api_error() {
        assert_eq!(LithicError::NotFound(BASE_ERROR.into()), LithicError::from(ApiError::NotFound(BASE_ERROR.into())));
        assert_eq!(LithicError::Unauthorized(BASE_ERROR.into()), LithicError::from(ApiError::Unauthorized(BASE_ERROR.into())));
        assert_eq!(LithicError::Conflict(BASE_ERROR.into()), LithicError::from(ApiError::Conflict(BASE_ERROR.into())));
        assert_eq!(LithicError::Unexpected(BASE_ERROR.into()), LithicError::from(ApiError::BadRequest(BASE_ERROR.into())));
        assert_eq!(LithicError::Unexpected(BASE_ERROR.into()), LithicError::from(ApiError::InternalServerError(BASE_ERROR.into())));
        assert_eq!(LithicError::Unexpected(BASE_ERROR.into()), LithicError::from(ApiError::Unexpected(BASE_ERROR.into())));
        assert_eq!(LithicError::Unexpected(BASE_ERROR.into()), LithicError::from(ApiError::Timeout(BASE_ERROR.into())));

    }
}