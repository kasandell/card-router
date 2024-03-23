use serde::Deserialize;
use serde_json::Error as SerdeError;
use thiserror;
use crate::error::api_error::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum CheckoutError {
    #[error("Unauthorized checkout attempt")]
    UnauthorizedCheckoutAttempt(#[source] Box<dyn std::error::Error>),
    #[error("Duplicate checkout attempts")]
    DuplicateCheckoutError(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected checkout error")]
    UnexpectedCheckoutError(#[source] Box<dyn std::error::Error>)
}

impl From<ApiError> for CheckoutError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Unauthorized(e) => CheckoutError::UnauthorizedCheckoutAttempt(e),
            ApiError::NotFound(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::BadRequest(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::Conflict(e) => CheckoutError::DuplicateCheckoutError(e),
            ApiError::InternalServerError(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::Timeout(e) => CheckoutError::UnexpectedCheckoutError(e),
            ApiError::Unexpected(e) => CheckoutError::UnexpectedCheckoutError(e),
        }
    }
}

impl From<SerdeError> for CheckoutError {
    fn from(value: SerdeError) -> Self {
        CheckoutError::UnexpectedCheckoutError(value.into())
    }
}

#[cfg(test)]
mod test {
    use crate::adyen::checkout::error::CheckoutError;
    use crate::error::api_error::ApiError;
    use serde_json::Error as SerdeError;

    #[test]
    pub fn test_api_error_mappings() {
        let base_error = "test";
        /*
        assert_eq!(CheckoutError::UnauthorizedCheckoutAttempt(base_error.clone().into()), CheckoutError::from(ApiError::Unauthorized(base_error.clone().into())));
        assert_eq!(CheckoutError::UnexpectedCheckoutError(base_error.clone().into()), CheckoutError::from(ApiError::NotFound(base_error.clone().into())));
        assert_eq!(CheckoutError::UnexpectedCheckoutError(base_error.clone().into()), CheckoutError::from(ApiError::BadRequest(base_error.clone().into())));
        assert_eq!(CheckoutError::DuplicateCheckoutError(base_error.clone().into()), CheckoutError::from(ApiError::Conflict(base_error.clone().into())));
        assert_eq!(CheckoutError::UnexpectedCheckoutError(base_error.clone().into()), CheckoutError::from(ApiError::InternalServerError(base_error.clone().into())));
        assert_eq!(CheckoutError::UnexpectedCheckoutError(base_error.clone().into()), CheckoutError::from(ApiError::Timeout(base_error.clone().into())));
        assert_eq!(CheckoutError::UnexpectedCheckoutError(base_error.clone().into()), CheckoutError::from(ApiError::Unexpected(base_error.clone().into())));
         */
    }

    #[test]
    pub fn test_serde_error_mappings() {
        let base_error = "test";
        /*
        assert_eq!(CheckoutError::UnexpectedCheckoutError(base_error.clone().into()), CheckoutError::from(SerdeError::from(base_error.clone().into())));
         */
    }
}