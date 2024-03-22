use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Unexpected")]
    Unexpected(#[source] Box<dyn std::error::Error>),
}

impl From<DataError> for CategoryError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => CategoryError::Unexpected(Box::new(e)),
            DataError::NotFound(e) => CategoryError::Unexpected(Box::new(e)),
            DataError::Format(e) => CategoryError::Unexpected(Box::new(e)),
            DataError::Unexpected(e) => CategoryError::Unexpected(Box::new(e)),
        }
    }
}