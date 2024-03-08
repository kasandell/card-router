use actix_web::http::StatusCode;
use derive_more::Display;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ErrorType {
    #[display(fmt = "Unauthorized")]
    Unauthorized = 401,
    #[display(fmt = "Conflict")]
    Conflict = 409,
    #[display(fmt = "Not Found")]
    NotFound = 404,
    #[display(fmt = "Client Error")]
    BadRequest = 400,
    #[display(fmt = "Internal Server Error")]
    InternalServerError = 500
}

impl From<ErrorType> for StatusCode {
    fn from(error_type: ErrorType) -> StatusCode {
        return match error_type {
            ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorType::Conflict => StatusCode::CONFLICT,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::BadRequest => StatusCode::BAD_REQUEST,
            ErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}