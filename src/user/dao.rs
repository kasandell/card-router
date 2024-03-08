use uuid::Uuid;
use crate::data_error::DataError;
use crate::user::entity::{User, UserMessage};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserDaoTrait {
    async fn find_all(&self) -> Result<Vec<User>, DataError>;
    async fn find(&self, id: &Uuid) -> Result<User, DataError>;
    async fn find_by_email_password(
        &self,
        email: &str,
        password: &str
    ) -> Result<User, DataError>;
    async fn find_by_internal_id(&self, id: i32) -> Result<User, DataError>;
    async fn find_by_auth0_id(&self, auth0_id: &str) -> Result<User, DataError>;
    async fn create<'a>(&self, user: &UserMessage<'a>) -> Result<User, DataError>;
    async fn update<'a>(&self, id: &Uuid, user: &UserMessage<'a>) -> Result<User, DataError>;
}

pub struct UserDao {}

impl UserDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserDaoTrait for UserDao {
    async fn find_all(&self) -> Result<Vec<User>, DataError> {
        User::find_all().await
    }

    async fn find(&self, id: &Uuid) -> Result<User, DataError> {
        User::find(id).await
    }

    async fn find_by_auth0_id(&self, auth0_id: &str) -> Result<User, DataError> {
        User::find_by_auth0_id(auth0_id).await
    }

    async fn find_by_email_password(
        &self,
        email: &str,
        password: &str
    ) -> Result<User, DataError> {
        User::find_by_email_password(email, password).await
    }

    async fn find_by_internal_id(&self, id: i32) -> Result<User, DataError> {
        User::find_by_internal_id(id).await
    }

    async fn create<'a>(&self, user: &UserMessage<'a>) -> Result<User, DataError> {
        User::create(user).await
    }

    async fn update<'a>(&self, id: &Uuid, user: &UserMessage<'a>) -> Result<User, DataError> {
        User::update(id, user).await
    }
}