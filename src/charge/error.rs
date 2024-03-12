use std::fmt;
use serde::Deserialize;
use crate::ledger::error::Error as LedgerError;

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

impl From<LedgerError> for Error {
    fn from(error: LedgerError) -> Error {
        Error::new(error.message)
    }
}
