use std::fmt;
use std::num::ParseIntError;
use serde::Deserialize;
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError;
use r2d2::Error as R2D2Error;
use serde_json::{json, Error as SerdeError};
use crate::api_error::ApiError;
use crate::error_type::ErrorType;

#[derive(Debug, Clone)]
pub struct DataError {
    pub error_type: ErrorType,
    pub message: String
}

impl DataError {
    pub fn new(error_type: ErrorType, message: &str) -> DataError {
        DataError { error_type, message: message.to_string() }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<R2D2Error> for DataError {
    fn from(_: R2D2Error) -> DataError {
        DataError::new(ErrorType::InternalServerError, "R2D2 error")
    }

}

impl From<RunError> for DataError {
    fn from(_: RunError) -> DataError {
        DataError::new(ErrorType::InternalServerError, "BB8 error")
    }

}

impl From<DieselError> for DataError {
    fn from(error: DieselError) -> DataError {
        info!("Converting from diesel error");
        match error {
            DieselError::DatabaseError(kind, err) => {
                /*
                    ForeignKeyViolation,
                    UnableToSendCommand,
                    SerializationFailure,
                    ReadOnlyTransaction,
                    NotNullViolation,
                    CheckViolation,
                    ClosedConnection,
                 */
                match kind {
                    _UniqueViolation => DataError::new(ErrorType::Conflict, err.message()),
                    _ => DataError::new(ErrorType::InternalServerError, err.message())
                }

            },
            DieselError::NotFound => DataError::new(ErrorType::NotFound, "Record not found"),
            err => DataError::new(ErrorType::InternalServerError, &format!("Diesel error: {}", err)),
        }
    }
}

impl From<SerdeError> for DataError {
    fn from(error: SerdeError) -> DataError {
        info!("Converting from serde error");
        match error {
            err => DataError::new(ErrorType::InternalServerError, &format!("Serde Error error: {}", err)),
        }
    }
}

impl From<ParseIntError> for DataError {
    fn from(_: ParseIntError) -> Self {
        DataError::new(ErrorType::InternalServerError, "Parse error")
    }
}

