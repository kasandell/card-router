use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Authorization error")]
    Unauthorized(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected auth error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}


impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AuthError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

}
