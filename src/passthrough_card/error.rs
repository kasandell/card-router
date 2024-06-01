use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum PassthroughCardError {
    #[error("Active card already exists")]
    ActiveCardExists(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unable to issue card")]
    IssueCard(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unable to transition status")]
    StatusUpdate(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Card not found")]
    CardNotFound(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)
}

impl ResponseError for PassthroughCardError {
    fn status_code(&self) -> StatusCode {
        match self {
            PassthroughCardError::ActiveCardExists(_) => StatusCode::CONFLICT,
            PassthroughCardError::IssueCard(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PassthroughCardError::StatusUpdate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PassthroughCardError::CardNotFound(_) => StatusCode::NOT_FOUND,
            PassthroughCardError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<DataError> for PassthroughCardError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => PassthroughCardError::ActiveCardExists(e),
            DataError::NotFound(e) => PassthroughCardError::CardNotFound(e),
            DataError::Format(e) => PassthroughCardError::Unexpected(e),
            DataError::Unexpected(e) => PassthroughCardError::Unexpected(e),
        }
    }
}

#[cfg(test)]
impl PartialEq for PassthroughCardError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PassthroughCardError::ActiveCardExists(_), PassthroughCardError::ActiveCardExists(_))
            | (PassthroughCardError::CardNotFound(_), PassthroughCardError::CardNotFound(_))
            | (PassthroughCardError::Unexpected(_), PassthroughCardError::Unexpected(_))
            | (PassthroughCardError::IssueCard(_), PassthroughCardError::IssueCard(_))
            | (PassthroughCardError::StatusUpdate(_), PassthroughCardError::StatusUpdate(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::ResponseError;
    use actix_web::http::StatusCode;
    use crate::error::data_error::DataError;
    use crate::passthrough_card::error::PassthroughCardError;

    const BASE_ERROR: &str = "test";

    #[test]
    pub fn test_from_data_error() {
        assert_eq!(PassthroughCardError::Unexpected(BASE_ERROR.into()), PassthroughCardError::from(DataError::Unexpected(BASE_ERROR.into())));
        assert_eq!(PassthroughCardError::CardNotFound(BASE_ERROR.into()), PassthroughCardError::from(DataError::NotFound(BASE_ERROR.into())));
        assert_eq!(PassthroughCardError::ActiveCardExists(BASE_ERROR.into()), PassthroughCardError::from(DataError::Conflict(BASE_ERROR.into())));
        assert_eq!(PassthroughCardError::Unexpected(BASE_ERROR.into()), PassthroughCardError::from(DataError::Format(BASE_ERROR.into())));
    }

    #[test]
    pub fn test_status_code() {
        assert_eq!(StatusCode::CONFLICT, PassthroughCardError::ActiveCardExists(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, PassthroughCardError::IssueCard(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, PassthroughCardError::StatusUpdate(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, PassthroughCardError::Unexpected(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::NOT_FOUND, PassthroughCardError::CardNotFound(BASE_ERROR.into()).status_code());

    }
}