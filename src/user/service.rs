use std::sync::Arc;
use async_trait::async_trait;
use crate::error::error_type::ErrorType;
use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::error::service_error::ServiceError;
use crate::user::dao::{UserDao, UserDaoTrait};
use crate::user::entity::{User, UserMessage};

#[async_trait(?Send)]
pub trait UserServiceTrait {
    async fn get_or_create(self: Arc<Self>, auth0_user_id: &str, email: &str) -> Result<User, ServiceError>;
}

pub struct UserService {
    pub user_dao: Arc<dyn UserDaoTrait>,
    pub footprint_service: Arc<dyn FootprintServiceTrait>
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_dao: Arc::new(UserDao::new()),
            footprint_service: Arc::new(FootprintService::new())
        }
    }

    pub fn new_with_services(
        user_dao: Arc<dyn UserDaoTrait>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            user_dao,
            footprint_service
        }
    }
}

#[async_trait(?Send)]
impl UserServiceTrait for UserService {
    async fn get_or_create(self: Arc<Self>, auth0_user_id: &str, email: &str) -> Result<User, ServiceError> {
        let res = self.user_dao.clone().find_by_auth0_id(auth0_user_id).await;
        match res {
            Ok(user) => Ok(user),
            Err(error) => {
                match &error.error_type {
                    // not found, can create user
                    ErrorType::NotFound => {
                        let footprint_vault_id = self.footprint_service.clone().add_vault_for_user().await?;
                        return Ok(
                            self.user_dao.clone().create(
                                &UserMessage {
                                    email,
                                    auth0_user_id,
                                    footprint_vault_id: &footprint_vault_id.id
                                }
                            ).await?
                        )
                    }
                    _ => Err(ServiceError::from(error))
                }
            }
        }
    }
}