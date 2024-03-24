use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
pub enum RuleError {
    #[error("No amount provided")]
    NoAmount(#[source] Box<dyn std::error::Error>),
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

#[cfg(test)]
impl PartialEq for RuleError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleError::NoAmount(_), RuleError::NoAmount(_))
            | (RuleError::Unexpected(_), RuleError::Unexpected(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use crate::rule::error::RuleError;

    const BASE_ERROR: &str = "test";

    #[test]
    pub fn test_status_codes() {
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, RuleError::Unexpected(BASE_ERROR.into()).status_code());
        assert_eq!(StatusCode::BAD_REQUEST, RuleError::NoAmount(BASE_ERROR.into()).status_code());
    }
}