use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum WalletError {
    #[error("User is not the owner of specified card")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Wallet not found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Card already matched")]
    Conflict(#[source] Box<dyn std::error::Error>),
    #[error("Unacceptable action")]
    NotAcceptable(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl ResponseError for WalletError {
    fn status_code(&self) -> StatusCode {
        match self {
            WalletError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            WalletError::NotFound(_) => StatusCode::NOT_FOUND,
            WalletError::Conflict(_) => StatusCode::CONFLICT,
            WalletError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WalletError::NotAcceptable(_) => StatusCode::NOT_ACCEPTABLE,
        }
    }
}

impl From<DataError> for WalletError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => WalletError::Conflict(e),
            DataError::NotFound(e) => WalletError::NotFound(e),
            DataError::Format(e) => WalletError::Unexpected(e),
            DataError::Unexpected(e) => WalletError::Unexpected(e),
        }
    }
}

#[cfg(test)]
impl PartialEq for WalletError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WalletError::Conflict(_), WalletError::Conflict(_))
            | (WalletError::NotFound(_), WalletError::NotFound(_))
            | (WalletError::Unexpected(_), WalletError::Unexpected(_))
            | (WalletError::NotAcceptable(_), WalletError::NotAcceptable(_))
            | (WalletError::Unauthorized(_), WalletError::Unauthorized(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use crate::error::data_error::DataError;
    use crate::wallet::error::WalletError;

    const BASE_ERROR: &str = "test";

    #[test]
    pub fn test_from_data_error() {
        assert_eq!(WalletError::Unexpected(BASE_ERROR.into()), WalletError::from(DataError::Unexpected(BASE_ERROR.into())));
        assert_eq!(WalletError::NotFound(BASE_ERROR.into()), WalletError::from(DataError::NotFound(BASE_ERROR.into())));
        assert_eq!(WalletError::Unexpected(BASE_ERROR.into()), WalletError::from(DataError::Format(BASE_ERROR.into())));
        assert_eq!(WalletError::Conflict(BASE_ERROR.into()), WalletError::from(DataError::Conflict(BASE_ERROR.into())));
    }

    #[test]
    pub fn test_status_codes() {
        assert_eq!(StatusCode::NOT_FOUND, WalletError::NotFound(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::UNAUTHORIZED, WalletError::Unauthorized(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, WalletError::Unexpected(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::CONFLICT, WalletError::Conflict(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::NOT_ACCEPTABLE, WalletError::NotAcceptable(BASE_ERROR.into()).status_code());
    }
}