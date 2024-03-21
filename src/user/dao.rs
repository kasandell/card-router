use uuid::Uuid;
use crate::error::error::ServiceError;
use crate::user::entity::{User, UserMessage};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserDaoTrait {
    async fn find(&self, id: &Uuid) -> Result<User, ServiceError>;
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<User, ServiceError>;
    async fn find_by_internal_id(&self, id: i32) -> Result<User, ServiceError>;
    async fn find_by_auth0_id(&self, auth0_id: &str) -> Result<User, ServiceError>;
    async fn create<'a>(&self, user: &UserMessage<'a>) -> Result<User, ServiceError>;
    async fn update<'a>(&self, id: &Uuid, user: &UserMessage<'a>) -> Result<User, ServiceError>;
}

pub struct UserDao {}

impl UserDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserDaoTrait for UserDao {
    async fn find(&self, id: &Uuid) -> Result<User, ServiceError> {
        User::find(id).await
    }

    async fn find_by_auth0_id(&self, auth0_id: &str) -> Result<User, ServiceError> {
        User::find_by_auth0_id(auth0_id).await
    }

    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<User, ServiceError> {
        User::find_by_email(email).await
    }

    async fn find_by_internal_id(&self, id: i32) -> Result<User, ServiceError> {
        User::find_by_internal_id(id).await
    }

    async fn create<'a>(&self, user: &UserMessage<'a>) -> Result<User, ServiceError> {
        User::create(user).await
    }

    async fn update<'a>(&self, id: &Uuid, user: &UserMessage<'a>) -> Result<User, ServiceError> {
        User::update(id, user).await
    }
}