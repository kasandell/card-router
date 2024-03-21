use std::fmt::Formatter;
use std::sync::Arc;
use lithic_client::models::Card;
use crate::error::error::ServiceError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::{LithicCard, PassthroughCard};
use crate::user::entity::User;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait PassthroughCardDaoTrait {
    async fn create(self: Arc<Self>, card: LithicCard, user: &User) -> Result<PassthroughCard, ServiceError>;
    async fn create_from_api_card(self: Arc<Self>, card: &Card, user: &User) -> Result<PassthroughCard, ServiceError>;
    async fn get(self: Arc<Self>, id: i32) -> Result<PassthroughCard, ServiceError>;
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCard, ServiceError>;
    async fn find_cards_for_user(self: Arc<Self>, user_id: i32) -> Result<Vec<PassthroughCard>, ServiceError>;
    async fn update_status(self: Arc<Self>, id: i32, status: PassthroughCardStatus) -> Result<PassthroughCard, ServiceError>;
    async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, ServiceError>;
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
    async fn create(self: Arc<Self>, card: LithicCard, user: &User) -> Result<PassthroughCard, ServiceError> {
        PassthroughCard::create(card, user).await
    }

    #[tracing::instrument(skip(self))]
    async fn create_from_api_card(self: Arc<Self>, card: &Card, user: &User) -> Result<PassthroughCard, ServiceError> {
        PassthroughCard::create_from_api_card(card, user).await
    }

    #[tracing::instrument(skip(self))]
    async fn get(self: Arc<Self>, id: i32) -> Result<PassthroughCard, ServiceError> {
        PassthroughCard::get(id).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCard, ServiceError> {
        PassthroughCard::get_by_token(token).await
    }

    #[tracing::instrument(skip(self))]
    async fn find_cards_for_user(self: Arc<Self>, user_id: i32) -> Result<Vec<PassthroughCard>, ServiceError> {
        PassthroughCard::find_cards_for_user(user_id).await
    }

    #[tracing::instrument(skip(self))]
    async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, ServiceError> {
        PassthroughCard::find_card_for_user_in_status(user_id, status).await
    }

    #[tracing::instrument(skip(self))]
    async fn update_status(self: Arc<Self>, id: i32, status: PassthroughCardStatus) -> Result<PassthroughCard, ServiceError> {
        PassthroughCard::update_status(id, status).await
    }
}