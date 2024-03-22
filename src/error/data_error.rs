use std::fmt::Debug;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use std::num::ParseIntError;
use diesel_async::pooled_connection::bb8::RunError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use r2d2::Error as R2D2Error;
use serde_json::{json, Error as SerdeError};
use thiserror;


#[derive(thiserror::Error, Debug)]
pub enum DataError {
    #[error("Conflict")]
    Conflict(#[source] Box<dyn std::error::Error>),
    #[error("Not Found")]
    NotFound(#[source] Box<dyn std::error::Error>),
    #[error("Format")]
    Format(#[source] Box<dyn std::error::Error>),
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error>),
}

impl From<R2D2Error> for DataError {
    fn from(e: R2D2Error) -> DataError {
        DataError::Unexpected(Box::new(e))
    }
}

impl From<RunError> for DataError {
    fn from(error: RunError) -> DataError {
        // TODO: this can also encapsulate duplicate, etc
        DataError::Unexpected(Box::new(error))
    }

}

impl From<DieselError> for DataError {
    fn from(error: DieselError) -> DataError {
        match &error {
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
                    DatabaseErrorKind::UniqueViolation => DataError::Conflict(Box::new(error)),
                    DatabaseErrorKind::SerializationFailure => DataError::Format(Box::new(error)),
                    _ => DataError::Unexpected(Box::new(error))
                }
            },
            DieselError::NotFound => DataError::NotFound(Box::new(error)),
            _ => DataError::Unexpected(Box::new(error))
        }
    }
}

impl From<SerdeError> for DataError {
    fn from(error: SerdeError) -> DataError {
        DataError::Format(Box::new(error))
    }
}

impl From<ParseIntError> for DataError {
    fn from(error: ParseIntError) -> Self {
        DataError::Format(Box::new(error))
    }
}

