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

#[cfg(test)]
impl PartialEq for FootprintError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FootprintError::Conflict(_), FootprintError::Conflict(_))
            | (FootprintError::BadRequest(_), FootprintError::BadRequest(_))
            | (FootprintError::Unexpected(_), FootprintError::Unexpected(_))
            | (FootprintError::NotFound(_), FootprintError::NotFound(_))
            | (FootprintError::NotImplemented, FootprintError::NotImplemented)
            | (FootprintError::Unauthorized(_), FootprintError::Unauthorized(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use serde::de::IntoDeserializer;
    use crate::error::api_error::ApiError;
    use crate::footprint::error::FootprintError;
    use crate::test_helper::error::serde_error;

    const BASE_ERROR: &str = "test";
    #[test]
    pub fn test_serde_error() {
        assert_eq!(FootprintError::BadRequest(BASE_ERROR.into()), FootprintError::from(serde_error()));
    }

    #[test]
    pub fn test_from_api_error() {
        assert_eq!(FootprintError::Unauthorized(BASE_ERROR.into()), FootprintError::from(ApiError::Unauthorized(BASE_ERROR.into())));
        assert_eq!(FootprintError::Conflict(BASE_ERROR.into()), FootprintError::from(ApiError::Conflict(BASE_ERROR.into())));
        assert_eq!(FootprintError::NotFound(BASE_ERROR.into()), FootprintError::from(ApiError::NotFound(BASE_ERROR.into())));
        assert_eq!(FootprintError::Unexpected(BASE_ERROR.into()), FootprintError::from(ApiError::BadRequest(BASE_ERROR.into())));
        assert_eq!(FootprintError::Unexpected(BASE_ERROR.into()), FootprintError::from(ApiError::InternalServerError(BASE_ERROR.into())));
        assert_eq!(FootprintError::Unexpected(BASE_ERROR.into()), FootprintError::from(ApiError::Timeout(BASE_ERROR.into())));
        assert_eq!(FootprintError::Unexpected(BASE_ERROR.into()), FootprintError::from(ApiError::Unexpected(BASE_ERROR.into())));
    }
}