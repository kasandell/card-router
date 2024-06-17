use std::sync::Arc;
use async_trait::async_trait;
use base64::prelude::*;
use crate::pagination::constant::SEPARATOR;
use crate::pagination::error::PaginationError;
use crate::pagination::model::PaginationModel;
use crate::pagination::r#type::{PaginatableType, PaginationLocation};

#[async_trait]
pub trait PaginationServiceTrait <T> where T: PaginatableType {
    async fn decode_cursor_to_service_and_id(self: Arc<Self>, cursor: String) -> Result<PaginationModel<T>, PaginationError>;
    async fn encode_cursor_for_service_and_cursor(self: Arc<Self>, service_name: String, column_name: String, cursor: T) -> Result<String, PaginationError>;
}

pub struct PaginationService {}

impl PaginationService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl <T> PaginationServiceTrait<T> for PaginationService where T: PaginatableType {
    async fn decode_cursor_to_service_and_id(self: Arc<Self>, cursor: String) -> Result<PaginationModel<T>, PaginationError> {
        let decoded = String::from_utf8(BASE64_STANDARD.decode(cursor)?)?;
        let split: Vec<&str> = decoded.split(SEPARATOR).collect();
        if split.len() != 3 {
            return Err(PaginationError::Unexpected("Invalid request body length".into()))
        }

        let service_name = split.get(0)
            .ok_or(PaginationError::Unexpected("Can't get first index".into()))?
            .to_string();

        let column_name = split.get(1)
            .ok_or(PaginationError::Unexpected("Can't get second index".into()))?
            .to_string();

        let cursor_location_string = split.get(2)
            .ok_or(PaginationError::Unexpected("Can't get second index".into()))?
            .to_string();

        let cursor_location: T = T::from_str(&cursor_location_string)
            .map_err(|e| PaginationError::Unexpected("Can't parse cursor string".into()))?;

        Ok(PaginationModel {
            service_name: service_name,
            column_name: column_name,
            cursor_location: cursor_location
        })
    }

    async fn encode_cursor_for_service_and_cursor(self: Arc<Self>, service_name: String, column_name: String, cursor: T) -> Result<String, PaginationError> {
        let cursor_str = format!("{}{}{}{}{}", service_name, SEPARATOR, column_name, SEPARATOR, cursor.to_string());
        let cursor_encoded = BASE64_STANDARD.encode(cursor_str);
        Ok(cursor_encoded)
    }
}


