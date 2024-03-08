use std::sync::Arc;
use async_trait::async_trait;
use crate::service_error::ServiceError;
use crate::user::dao::{UserDao, UserDaoTrait};
use crate::user::entity::{User, UserMessage};

#[async_trait(?Send)]
pub trait UserServiceTrait {
    async fn get_or_create(&self, auth0_user_id: &str, email: &str) -> Result<User, ServiceError>;
}

pub struct UserService {
    pub user_dao: Arc<dyn UserDaoTrait>
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_dao: Arc::new(UserDao::new())
        }
    }

    pub fn new_with_services(user_dao: Arc<dyn UserDaoTrait>) -> Self {
        Self {
            user_dao
        }
    }
}

#[async_trait(?Send)]
impl UserServiceTrait for UserService {
    async fn get_or_create(&self, auth0_user_id: &str, email: &str) -> Result<User, ServiceError> {
        let res = self.user_dao.clone().find_by_auth0_id(auth0_user_id).await;
        match res {
            Ok(user) => Ok(user),
            Err(error) => {
                match &error.status_code {
                    // not found, can create user
                    404 => {
                        return self.user_dao.clone().create(
                            &UserMessage {
                                email,
                                auth0_user_id,
                            }
                        ).await.map_err(|e| ServiceError::from(e))
                    }
                    _ => Err(ServiceError::from(error))
                }
            }
        }
    }
}