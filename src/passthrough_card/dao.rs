use std::sync::Arc;
use crate::error::data_error::DataError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::{InsertablePassthroughCard, PassthroughCard};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait PassthroughCardDaoTrait {
    async fn create(self: Arc<Self>, card: InsertablePassthroughCard) -> Result<PassthroughCard, DataError>;
    async fn get(self: Arc<Self>, id: i32) -> Result<PassthroughCard, DataError>;
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCard, DataError>;
    async fn find_cards_for_user(self: Arc<Self>, user_id: i32) -> Result<Vec<PassthroughCard>, DataError>;
    async fn update_status(self: Arc<Self>, id: i32, status: PassthroughCardStatus) -> Result<PassthroughCard, DataError>;
    async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, DataError>;
}


pub struct PassthroughCardDao {}

impl PassthroughCardDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl PassthroughCardDaoTrait for PassthroughCardDao {
    #[tracing::instrument(skip(self))]
    async fn create(self: Arc<Self>, card: InsertablePassthroughCard) -> Result<PassthroughCard, DataError> {
        PassthroughCard::create(card).await
    }

    #[tracing::instrument(skip(self))]
    async fn get(self: Arc<Self>, id: i32) -> Result<PassthroughCard, DataError> {
        PassthroughCard::get(id).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCard, DataError> {
        tracing::warn!("runtime: {:?}, task: {:?}", tokio::runtime::Handle::current().id(), tokio::task::id());
        PassthroughCard::get_by_token(token).await
    }

    #[tracing::instrument(skip(self))]
    async fn find_cards_for_user(self: Arc<Self>, user_id: i32) -> Result<Vec<PassthroughCard>, DataError> {
        PassthroughCard::find_cards_for_user(user_id).await
    }

    #[tracing::instrument(skip(self))]
    async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, DataError> {
        PassthroughCard::find_card_for_user_in_status(user_id, status).await
    }

    #[tracing::instrument(skip(self))]
    async fn update_status(self: Arc<Self>, id: i32, status: PassthroughCardStatus) -> Result<PassthroughCard, DataError> {
        PassthroughCard::update_status(id, status).await
    }
}