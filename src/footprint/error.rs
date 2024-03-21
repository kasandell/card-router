use actix_web::http::StatusCode;
use serde_json::Error as SerdeError;
use crate::error::api_error::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum FootprintError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("Footprint formatting error")]
    FootprintFormattingError(Box<dyn std::error::Error>),
    #[error("Unexpected footprint error")]
    UnexpectedFootprintError(Box<dyn std::error::Error>)

}


impl From<ApiError> for FootprintError {
    fn from(value: ApiError) -> Self {
        todo!()
        /*
        match value {
            ApiError::Unauthorized(_) => {}
            ApiError::NotFound(_) => {}
            ApiError::BadRequest(_) => {}
            ApiError::Conflict(_) => {}
            ApiError::InternalServerError(_) => {}
            ApiError::Timeout(_) => {}
            ApiError::Unexpected(_) => {}
        }
         */
    }
}
