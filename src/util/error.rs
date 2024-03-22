use std::num::ParseIntError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum UtilityError {
    #[error("Date Construction Error")]
    DateError(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected utility error")]
    Unexpected(#[source] Box<dyn std::error::Error>)
}

// TODO: REMOVE THIS
impl From<ParseIntError> for UtilityError {
    fn from(value: ParseIntError) -> Self {
        UtilityError::Unexpected(Box::new(value))
    }
}