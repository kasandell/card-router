use std::fmt;
use serde::Deserialize;
use adyen_checkout::apis::Error as AdyenCheckoutError;
use serde_json::Error as SerdeError;

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
