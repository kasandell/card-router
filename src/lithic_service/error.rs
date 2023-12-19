use std::fmt;
use serde::Deserialize;
use lithic_client::apis::Error as LithicError;
use serde_json::Error as SerdeError;

#[derive(Debug, Deserialize, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }
}

impl Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        match error {
            err => Error::new( format!("Serde Error error: {}", err)),
        }
    }
}

impl <T> From<LithicError<T>> for Error {
    fn from(error: LithicError<T>) -> Error {
        match error {
            err => Error::new(format!("Lithic error: {}", err)),
        }
    }
}
