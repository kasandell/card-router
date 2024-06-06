use std::sync::Arc;
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::user::entity::{User, UserMessage};
use async_trait::async_trait;
use diesel::row::NamedRow;
#[cfg(not(feature = "no-redis"))]
use crate::redis::services::{
    RedisService,
    RedisServiceTrait
};
#[cfg(not(feature = "no-redis"))]
use redis::Commands;
#[cfg(not(feature = "no-redis"))]
use crate::redis::helper::try_redis_fallback_db;
#[cfg(not(feature = "no-redis"))]
use crate::redis::key::Key;


#[async_trait(?Send)]
pub trait UserDaoTrait {
    async fn find(&self, id: &Uuid) -> Result<User, DataError>;
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<User, DataError>;
    async fn find_by_internal_id(&self, id: i32) -> Result<User, DataError>;
    async fn find_by_auth0_id(&self, auth0_id: &str) -> Result<User, DataError>;
    async fn create<'a>(&self, user: &UserMessage<'a>) -> Result<User, DataError>;
    async fn update<'a>(&self, id: &Uuid, user: &UserMessage<'a>) -> Result<User, DataError>;
}

pub struct UserDao {
    #[cfg(not(feature = "no-redis"))]
    redis: Arc<RedisService>
}

impl UserDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        #[cfg(not(feature = "no-redis"))] {
            Self {
                redis: Arc::new(RedisService::new())
            }
        }
        #[cfg(feature = "no-redis")] {
            Self {}
        }
    }
}

#[async_trait(?Send)]
impl UserDaoTrait for UserDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find(&self, id: &Uuid) -> Result<User, DataError> {
        User::find(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_by_auth0_id(&self, auth0_id: &str) -> Result<User, DataError> {
        User::find_by_auth0_id(auth0_id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<User, DataError> {
        User::find_by_email(email).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_by_internal_id(&self, id: i32) -> Result<User, DataError> {
        #[cfg(not(feature = "no-redis"))] {
            Ok(try_redis_fallback_db(
                self.redis.clone(),
                Key::User(id),
                || async {User::find_by_internal_id(id).await},
                true
            ).await?)
        }
        #[cfg(feature = "no-redis")] {
            User::find_by_internal_id(id).await
        }
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn create<'a>(&self, user: &UserMessage<'a>) -> Result<User, DataError> {
        User::create(user).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn update<'a>(&self, id: &Uuid, user: &UserMessage<'a>) -> Result<User, DataError> {
        User::update(id, user).await
    }
}