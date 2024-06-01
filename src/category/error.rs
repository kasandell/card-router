use crate::error::data_error::DataError;

#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Unexpected")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl From<DataError> for CategoryError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::Conflict(e) => CategoryError::Unexpected(e),
            DataError::NotFound(e) => CategoryError::Unexpected(e),
            DataError::Format(e) => CategoryError::Unexpected(e),
            DataError::Unexpected(e) => CategoryError::Unexpected(e),
        }
    }
}

#[cfg(test)]
impl PartialEq for CategoryError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CategoryError::Unexpected(_), CategoryError::Unexpected(_)) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::category::error::CategoryError;
    use crate::error::data_error::DataError;
    #[test]
    pub fn test_data_error_mappings() {
        let base_test = "test";
        assert_eq!(CategoryError::Unexpected(base_test.clone().into()), CategoryError::from(DataError::Conflict(base_test.clone().into())));
        assert_eq!(CategoryError::Unexpected(base_test.clone().into()), CategoryError::from(DataError::NotFound(base_test.clone().into())));
        assert_eq!(CategoryError::Unexpected(base_test.clone().into()), CategoryError::from(DataError::Format(base_test.clone().into())));
        assert_eq!(CategoryError::Unexpected(base_test.clone().into()), CategoryError::from(DataError::Unexpected(base_test.clone().into())));
    }
}
