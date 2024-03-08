use std::sync::Arc;
use async_trait::async_trait;
use crate::error_type::ErrorType;
use crate::footprint_service::response::AddVaultResponse;
use crate::service_error::ServiceError;
use crate::user::entity::User;

#[async_trait(?Send)]
pub trait FootprintServiceTrait {
    async fn add_vault_for_user(self: Arc<Self>) -> Result<AddVaultResponse, ServiceError>;
}

pub struct FootprintService {}

impl FootprintService {
    pub fn new() -> Self {
        Self {}
    }
}


#[async_trait(?Send)]
impl FootprintServiceTrait for FootprintService {
    async fn add_vault_for_user(self: Arc<Self>) ->  Result<AddVaultResponse, ServiceError> {
        Err(
            ServiceError::new(
                ErrorType::InternalServerError, "Not implemented"
            )
        )
    }

}