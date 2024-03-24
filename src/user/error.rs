use actix_web::{Responder, ResponseError};
use actix_web::http::StatusCode;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("Unauthorized User")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Not found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected user error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            UserError::NotFound(_) => StatusCode::NOT_FOUND,
            UserError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<DataError> for UserError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => UserError::Unexpected(e),
            DataError::NotFound(e) => UserError::NotFound(e),
            DataError::Format(e) => UserError::Unexpected(e),
            DataError::Unexpected(e) => UserError::Unexpected(e)
        }
    }
}


#[cfg(test)]
impl PartialEq for UserError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UserError::NotFound(_), UserError::NotFound(_))
            | (UserError::Unauthorized(_), UserError::Unauthorized(_))
            | (UserError::Unexpected(_), UserError::Unexpected(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use crate::error::data_error::DataError;
    use crate::user::error::UserError;

    const BASE_ERROR: &str = "test";
    #[test]
    pub fn test_from_data_error() {
        assert_eq!(UserError::NotFound(BASE_ERROR.into()), UserError::from(DataError::NotFound(BASE_ERROR.into())));
        assert_eq!(UserError::Unexpected(BASE_ERROR.into()), UserError::from(DataError::Conflict(BASE_ERROR.into())));
        assert_eq!(UserError::Unexpected(BASE_ERROR.into()), UserError::from(DataError::Format(BASE_ERROR.into())));
        assert_eq!(UserError::Unexpected(BASE_ERROR.into()), UserError::from(DataError::Unexpected(BASE_ERROR.into())));
    }

    #[test]
    pub fn test_status_codes() {
        assert_eq!(StatusCode::NOT_FOUND, UserError::NotFound(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::UNAUTHORIZED, UserError::Unauthorized(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, UserError::Unexpected(BASE_ERROR.into()).status_code());
    }
}