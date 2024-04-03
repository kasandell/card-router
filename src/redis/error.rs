use bb8::RunError;
use redis::{ErrorKind, RedisError as RedisClientError};
use crate::redis::error::RedisError::{NotFound, Unexpected};
use serde_json::Error as SerdeError;

#[derive(thiserror::Error, Debug)]
pub enum RedisError {
    #[error("Not found for type in redis")]
    NotFound(Box<dyn std::error::Error>),
    #[error("Unexpected error occurred")]
    Unexpected(Box<dyn std::error::Error>)
}

impl From<SerdeError> for RedisError {
    fn from(value: SerdeError) -> Self {
        // TODO: if we request an object of the wrong type, we come here. we can do better than unexpected
        RedisError::Unexpected(value.into())
    }
}

impl<T> From<RunError<T>> for RedisError {
    fn from(value: RunError<T>) -> Self {
        RedisError::Unexpected("Run error".into())
    }
}

impl From<RedisClientError> for RedisError {
    fn from(value: RedisClientError) -> Self {
        match value.kind() {
            /*
            ErrorKind::ResponseError => {}
            ErrorKind::ParseError => {}
            ErrorKind::AuthenticationFailed => {}
            ErrorKind::TypeError => {}
            ErrorKind::ExecAbortError => {}
            ErrorKind::BusyLoadingError => {}
            ErrorKind::NoScriptError => {}
            ErrorKind::InvalidClientConfig => {}
            ErrorKind::Moved => {}
            ErrorKind::Ask => {}
            ErrorKind::TryAgain => {}
            ErrorKind::ClusterDown => {}
            ErrorKind::CrossSlot => {}
            ErrorKind::MasterDown => {}
            ErrorKind::IoError => {}
            ErrorKind::ClientError => {}
            ErrorKind::ExtensionError => {}
            ErrorKind::ReadOnly => {}
            ErrorKind::MasterNameNotFoundBySentinel => {}
            ErrorKind::NoValidReplicasFoundBySentinel => {}
            ErrorKind::EmptySentinelList => {}
            ErrorKind::NotBusy => {}
            ErrorKind::ClusterConnectionNotFound => {}
            ErrorKind::Serialize => {}
             */
            ErrorKind::TypeError => {
                println!("converting from {:?}", value.kind());
                NotFound(value.into())
            },
            _ => {
                println!("converting from {:?}", value.kind());
                Unexpected(value.into())
            }
        }
    }
}

#[cfg(test)]
impl PartialEq for RedisError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Unexpected(_), Unexpected(_))
            | (NotFound(_), NotFound(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::redis::error::RedisError;
    use crate::test_helper::error::serde_error;

    #[test]
    fn test_from_serde() {
        assert_eq!(RedisError::Unexpected("test".into()), RedisError::from(serde_error()));
    }
}