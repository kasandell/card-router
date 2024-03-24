use std::sync::Arc;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use crate::error::data_error::DataError;
use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::user::dao::{UserDao, UserDaoTrait};
use crate::user::entity::{User, UserMessage};
use crate::user::model::UserModel;
use super::error::UserError;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UserServiceTrait {
    async fn get_or_create(self: Arc<Self>, auth0_user_id: &str, email: &str) -> Result<UserModel, UserError>;
    async fn find_by_internal_id(&self, id: i32) -> Result<UserModel, UserError>;
}

pub struct UserService {
    pub user_dao: Arc<dyn UserDaoTrait>,
    pub footprint_service: Arc<dyn FootprintServiceTrait>
}

impl UserService {

    #[tracing::instrument(skip_all)]
    pub fn new_with_services(
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            user_dao: Arc::new(UserDao::new()),
            footprint_service
        }
    }
}

#[cfg(test)]
impl UserService {
    #[tracing::instrument(skip_all)]
    pub fn new_with_mocks(
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
    #[tracing::instrument(skip(self))]
    async fn get_or_create(self: Arc<Self>, auth0_user_id: &str, email: &str) -> Result<UserModel, UserError> {
        let res = self.user_dao.clone().find_by_auth0_id(auth0_user_id).await;
        match res {
            Ok(user) => Ok(user.into()),
            Err(error) => {
                match &error {
                    // not found, can create user
                    DataError::NotFound(_)=> {
                        let footprint_vault_id = self.footprint_service.clone().add_vault_for_user().await
                            .map_err(|e| UserError::Unexpected(Box::new(e)))?;
                        return Ok(
                            self.user_dao.clone().create(
                                &UserMessage {
                                    email,
                                    auth0_user_id,
                                    footprint_vault_id: &footprint_vault_id.id
                                }
                            ).await.map_err(|e| UserError::Unexpected(Box::new(e)))?.into()
                        )
                    }
                    _ => Err(UserError::Unexpected(Box::new(error)))
                }
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_internal_id(&self, id: i32) -> Result<UserModel, UserError> {
        tracing::warn!("runtime: {:?}, task: {:?}", tokio::runtime::Handle::current().id(), tokio::task::id());
        Ok(self.user_dao.clone().find_by_internal_id(id)
            .await.map_err(|e| UserError::Unexpected(e.into()))?.into())
    }
}