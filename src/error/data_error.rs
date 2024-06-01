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
    Conflict(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Not Found")]
    NotFound(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Format")]
    Format(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Unexpected error")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl From<R2D2Error> for DataError {
    fn from(e: R2D2Error) -> DataError {
        DataError::Unexpected(Box::new(e))
    }
}

impl From<RunError> for DataError {
    fn from(error: RunError) -> DataError {
        // TODO: this can also encapsulate duplicate, etc
        tracing::info!("DataError from run error={:?}", &error);
        DataError::Unexpected(Box::new(error))
    }

}

impl From<DieselError> for DataError {
    fn from(error: DieselError) -> DataError {
        tracing::info!("DataError from diesel error={:?}", &error);
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
        tracing::info!("DataError from serde error={:?}", &error);
        DataError::Format(Box::new(error))
    }
}

impl From<ParseIntError> for DataError {
    fn from(error: ParseIntError) -> Self {
        tracing::info!("DataError from parse int error={:?}", &error);

        DataError::Format(Box::new(error))
    }
}


#[cfg(test)]
impl PartialEq for DataError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataError::Conflict(_), DataError::Conflict(_))
            | (DataError::NotFound(_), DataError::NotFound(_))
            | (DataError::Unexpected(_), DataError::Unexpected(_))
            | (DataError::Format(_), DataError::Format(_)) => true,
            _ => false
        }
    }
}


#[cfg(test)]
mod test {
    use crate::error::data_error::DataError;
    use crate::test_helper::error::serde_error;

    #[test]
    pub fn test_parse_int() {
        let base_error = "test";
        assert_eq!(DataError::Format(base_error.into()), DataError::from("hi".parse::<i32>().expect_err("shouldn't parse")));
    }

    #[test]
    pub fn test_serde_error() {
        let base_error = "test";
        assert_eq!(DataError::Format(base_error.into()), DataError::from(serde_error()));
    }
}