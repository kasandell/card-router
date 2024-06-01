use std::num::ParseIntError;
use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum UtilityError {
    #[error("Date Construction Error")]
    DateError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unexpected utility error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>)
}

#[cfg(test)]
impl PartialEq for UtilityError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UtilityError::Unexpected(_), UtilityError::Unexpected(_))
            | (UtilityError::DateError(_), UtilityError::DateError(_)) => true,
            _ => false
        }
    }
}