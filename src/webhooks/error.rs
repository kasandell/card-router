use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::passthrough_card::error::PassthroughCardError;
use crate::wallet::error::WalletError;

#[derive(thiserror::Error, Debug)]
pub enum LithicHandlerError {
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)
}

impl ResponseError for LithicHandlerError {
    fn status_code(&self) -> StatusCode {
        match self {
            LithicHandlerError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// TODO: probably maperr in code rather than overarching conversion
impl From<PassthroughCardError> for LithicHandlerError {
    fn from(value: PassthroughCardError) -> Self {
        LithicHandlerError::Unexpected(Box::new(value))
    }
}

impl From<WalletError> for LithicHandlerError {
    fn from(value: WalletError) -> Self {
        LithicHandlerError::Unexpected(Box::new(value))
    }
}

#[cfg(test)]
impl PartialEq for LithicHandlerError {
    fn eq(&self, other: &Self) -> bool {
        match(self, other) {
            (LithicHandlerError::Unexpected(_), LithicHandlerError::Unexpected(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use crate::passthrough_card::error::PassthroughCardError;
    use crate::wallet::error::WalletError;
    use crate::webhooks::error::LithicHandlerError;

    const BASE_ERROR: &str = "test";

    #[test]
    pub fn test_from_wallet() {
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(WalletError::Unexpected(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(WalletError::Conflict(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(WalletError::Unauthorized(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(WalletError::NotFound(BASE_ERROR.into())));
    }

    #[test]
    pub fn test_from_passthrough() {
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(PassthroughCardError::Unexpected(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(PassthroughCardError::IssueCard(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(PassthroughCardError::StatusUpdate(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(PassthroughCardError::ActiveCardExists(BASE_ERROR.into())));
        assert_eq!(LithicHandlerError::Unexpected(BASE_ERROR.into()), LithicHandlerError::from(PassthroughCardError::CardNotFound(BASE_ERROR.into())));
    }

    #[test]
    pub fn test_status_code() {
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, LithicHandlerError::Unexpected(BASE_ERROR.into()).status_code());
    }
}