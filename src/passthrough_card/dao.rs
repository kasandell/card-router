use std::sync::Arc;
use crate::error::data_error::DataError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::{InsertablePassthroughCard, PassthroughCard};
use async_trait::async_trait;
#[cfg(not(feature = "no-redis"))]
use crate::redis::helper::try_redis_fallback_db;
#[cfg(not(feature = "no-redis"))]
use crate::redis::key::Key;
#[cfg(not(feature = "no-redis"))]
use crate::redis::services::{
    RedisService,
    RedisServiceTrait
};



#[async_trait(?Send)]
pub trait PassthroughCardDaoTrait {
    async fn create(self: Arc<Self>, card: InsertablePassthroughCard) -> Result<PassthroughCard, DataError>;
    async fn get(self: Arc<Self>, id: i32) -> Result<PassthroughCard, DataError>;
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCard, DataError>;
    async fn find_cards_for_user(self: Arc<Self>, user_id: i32) -> Result<Vec<PassthroughCard>, DataError>;
    async fn update_status(self: Arc<Self>, card: &PassthroughCard, status: PassthroughCardStatus) -> Result<PassthroughCard, DataError>;
    async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, DataError>;
}


pub struct PassthroughCardDao {
    #[cfg(not(feature = "no-redis"))]
    redis: Arc<RedisService>
}

impl PassthroughCardDao {
    pub fn new() -> Self {
        #[cfg(not(feature = "no-redis"))]
        {
            Self {
                redis: Arc::new(RedisService::new())
            }
        }
        #[cfg(feature = "no-redis")]
        {
            Self {}
        }
    }
}


#[async_trait(?Send)]
impl PassthroughCardDaoTrait for PassthroughCardDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn create(self: Arc<Self>, card: InsertablePassthroughCard) -> Result<PassthroughCard, DataError> {
        PassthroughCard::create(card).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get(self: Arc<Self>, id: i32) -> Result<PassthroughCard, DataError> {
        PassthroughCard::get(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCard, DataError> {
        #[cfg(not(feature = "no-redis"))]
        {
            Ok(try_redis_fallback_db(
                self.redis.clone(),
                Key::PassthroughCardByToken(token),
                || async { PassthroughCard::get_by_token(token).await },
                false
            ).await?)
        }
        #[cfg(feature = "no-redis")]
        {

            PassthroughCard::get_by_token(token).await
        }
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_cards_for_user(self: Arc<Self>, user_id: i32) -> Result<Vec<PassthroughCard>, DataError> {
        PassthroughCard::find_cards_for_user(user_id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, DataError> {
        PassthroughCard::find_card_for_user_in_status(user_id, status).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn update_status(self: Arc<Self>, card: &PassthroughCard, status: PassthroughCardStatus) -> Result<PassthroughCard, DataError> {
        #[cfg(not(feature = "no-redis"))]
        {
            self.redis.clone().expire_now(&Key::PassthroughCardByToken(card.token.as_str())).await;
            PassthroughCard::update_status(card.id, status).await
        }
        #[cfg(feature = "no-redis")]
        {
            PassthroughCard::update_status(card.id, status).await
        }
    }
}