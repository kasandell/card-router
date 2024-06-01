use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Authorization error")]
    Unauthorized(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unexpected auth error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)
}


impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AuthError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

}


#[cfg(test)]
mod test {
    use actix_web::ResponseError;
    use actix_web::http::StatusCode;
    use crate::auth::error::AuthError;

    #[test]
    pub fn test_status_code_for_auth_error() {
        let base_error = "test";
        assert_eq!(StatusCode::UNAUTHORIZED, AuthError::Unauthorized(base_error.clone().into()).status_code());
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, AuthError::Unexpected(base_error.clone().into()).status_code());
    }
}