use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum PassthroughCardError {
    #[error("Active card already exists")]
    ActiveCardExists(#[source] Box<dyn std::error::Error>),
    #[error("Unable to issue card")]
    IssueCard(#[source] Box<dyn std::error::Error>),
    #[error("Unable to transition status")]
    StatusUpdate(#[source] Box<dyn std::error::Error>),
    #[error("Card not found")]
    CardNotFound(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
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
