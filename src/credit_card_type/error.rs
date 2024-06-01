use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum CreditCardTypeError {
   #[error("Unexpected Error")]
   Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)
}

impl From<DataError> for CreditCardTypeError {
    fn from(value: DataError) -> Self {
        CreditCardTypeError::Unexpected(Box::new(value))
    }
}

impl ResponseError for CreditCardTypeError {
    fn status_code(&self) -> StatusCode {
        match self {
            CreditCardTypeError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}


#[cfg(test)]
impl PartialEq for CreditCardTypeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CreditCardTypeError::Unexpected(_), CreditCardTypeError::Unexpected(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::ResponseError;
    use actix_web::http::StatusCode;
    use crate::credit_card_type::error::CreditCardTypeError;
    use crate::error::data_error::DataError;

    #[test]
    pub fn test_status_code() {
        let base_error = "test";
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, CreditCardTypeError::Unexpected(base_error.into()).status_code());
    }

    #[test]
    pub fn test_from_data_error() {
        let base_error = "test";
        assert_eq!(CreditCardTypeError::Unexpected(base_error.into()), CreditCardTypeError::from(DataError::Unexpected(base_error.into())))

    }
}