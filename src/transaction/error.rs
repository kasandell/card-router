use std::fmt;
use serde::Deserialize;
use crate::api_error::ApiError;
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;

#[derive(Debug, Deserialize)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }
}

impl From<R2D2Error> for Error {
    fn from(_: R2D2Error) -> Error {
        Error::new("R2D2 error".to_string())
    }

}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        info!("Converting from diesel error");
        match error {
            DieselError::DatabaseError(_, err) => Error::new(err.message().to_string()),
            DieselError::NotFound => Error::new("Record not found".to_string()),
            err => Error::new(format!("Diesel error: {}", err)),
        }
    }
}


