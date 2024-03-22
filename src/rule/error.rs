use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub enum RuleError<'a> {
    #[error("No amount provided")]
    NoAmount(&'a str),
    #[error("Unexpected Error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

impl ResponseError for RuleError {
    fn status_code(&self) -> StatusCode {
        match self {
            RuleError::NoAmount(_) => StatusCode::BAD_REQUEST,
            RuleError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}