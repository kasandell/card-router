use std::sync::Arc;
use crate::error::data_error::DataError;
use crate::user::model::UserModel as User;
use crate::wallet::entity::{InsertableCardAttempt, Wallet, WalletCardAttempt, UpdateCardAttempt, WalletDetail, InsertableCard, WalletWithExtraInfo};
use async_trait::async_trait;
use tracing;
#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::redis::helper::try_redis_fallback_db;
use crate::redis::key::Key;
use crate::redis::services::{
    RedisService,
    RedisServiceTrait
};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WalletDaoTrait {
    async fn find_all_for_user(self: Arc<Self>, user: &User) -> Result<Vec<Wallet>, DataError>;
    async fn find_all_for_user_with_card_info(self: Arc<Self>, user: &User) -> Result<Vec<WalletWithExtraInfo>, DataError>;
    async fn insert_card<'a>(self: Arc<Self>, card: &InsertableCard<'a>) -> Result<Wallet, DataError>;
}


#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WalletCardAttemtDaoTrait {
    async fn insert<'a>(self: Arc<Self>, card_attempt: &InsertableCardAttempt<'a>) -> Result<WalletCardAttempt, DataError>;
    async fn find_by_reference_id(self: Arc<Self>, reference: &str) -> Result<WalletCardAttempt, DataError>;
    async fn update_card(self: Arc<Self>, id: i32, card: &UpdateCardAttempt) -> Result<WalletCardAttempt, DataError>;


}

pub struct WalletDao {
    #[cfg(not(feature = "no-redis"))]
    redis: Arc<RedisService>
}
pub struct WalletCardAttemptDao {}

impl WalletDao {
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
impl WalletDaoTrait for WalletDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_all_for_user(self: Arc<Self>, user: &User) -> Result<Vec<Wallet>, DataError> {
        #[cfg(not(feature = "no-redis"))] {
            Ok(try_redis_fallback_db(
                self.redis.clone(),
                Key::CardsForUser(user.id),
                || async {Wallet::find_all_for_user(user).await},
                false
            ).await?)
        }
        #[cfg(feature = "no-redis")] {
            Wallet::find_all_for_user(user).await
        }
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_all_for_user_with_card_info(self: Arc<Self>, user: &User) -> Result<Vec<WalletWithExtraInfo>, DataError> {
        Wallet::find_all_for_user_with_card_info(user).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert_card<'a>(self: Arc<Self>, card: &InsertableCard<'a>) -> Result<Wallet, DataError> {
        let created_card = Wallet::insert_card(card).await;
        #[cfg(not(feature = "no-redis"))] {
            tracing::info!("Expiring user's wallet in redis for user_id={}", card.user_id);
            self.redis.clone().expire_now::<_>(&Key::CardsForUser(card.user_id)).await;
        }
        created_card
    }
}

impl WalletCardAttemptDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl WalletCardAttemtDaoTrait for WalletCardAttemptDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert<'a>(self: Arc<Self>, card_attempt: &InsertableCardAttempt<'a>) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::insert(card_attempt).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_by_reference_id(self: Arc<Self>, reference: &str) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::find_by_reference_id(reference).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn update_card(self: Arc<Self>, id: i32, card: &UpdateCardAttempt) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::update_card(id, card).await
    }
}

