use std::fmt;
use std::num::ParseIntError;
use serde::Deserialize;
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use serde_json::{json, Error as SerdeError};
use crate::api_error::ApiError;

#[derive(Debug, Deserialize, Clone)]
pub struct DataError {
    pub status_code: u16,
    pub message: String
}

impl DataError {
    pub fn new(status_code: u16, message: String) -> DataError {
        DataError { status_code, message }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<R2D2Error> for DataError {
    fn from(_: R2D2Error) -> DataError {
        DataError::new(500, "R2D2 error".to_string())
    }

}

impl From<DieselError> for DataError {
    fn from(error: DieselError) -> DataError {
        info!("Converting from diesel error");
        match error {
            DieselError::DatabaseError(_, err) => DataError::new(500, err.message().to_string()),
            DieselError::NotFound => DataError::new(404, "Record not found".to_string()),
            err => DataError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<SerdeError> for DataError {
    fn from(error: SerdeError) -> DataError {
        info!("Converting from serde error");
        match error {
            err => DataError::new(500, format!("Serde Error error: {}", err)),
        }
    }
}

impl From<ParseIntError> for DataError {
    fn from(_: ParseIntError) -> Self {
        DataError::new(500, "Parse error".to_string())
    }
}

