use lithic_client::models::Card;
use crate::data_error::DataError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::{LithicCard, PassthroughCard};
use crate::user::entity::User;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PassthroughCardDaoTrait {
    async fn create(&self, card: LithicCard, user: &User) -> Result<PassthroughCard, DataError>;
    async fn create_from_api_card(&self, card: &Card, user: &User) -> Result<PassthroughCard, DataError>;
    async fn get(&self, id: i32) -> Result<PassthroughCard, DataError>;
    async fn get_by_token(&self, token: String) -> Result<PassthroughCard, DataError>;
    async fn find_cards_for_user(&self, user_id: i32) -> Result<Vec<PassthroughCard>, DataError>;
    async fn find_card_for_user_in_status(
        &self,
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

#[async_trait]
impl PassthroughCardDaoTrait for PassthroughCardDao {
    async fn create(&self, card: LithicCard, user: &User) -> Result<PassthroughCard, DataError> {
        PassthroughCard::create(card, user).await
    }

    async fn create_from_api_card(&self, card: &Card, user: &User) -> Result<PassthroughCard, DataError> {
        PassthroughCard::create_from_api_card(card, user).await
    }

    async fn get(&self, id: i32) -> Result<PassthroughCard, DataError> {
        PassthroughCard::get(id).await
    }

    async fn get_by_token(&self, token: String) -> Result<PassthroughCard, DataError> {
        PassthroughCard::get_by_token(token).await
    }

    async fn find_cards_for_user(&self, user_id: i32) -> Result<Vec<PassthroughCard>, DataError> {
        PassthroughCard::find_cards_for_user(user_id).await
    }

    async fn find_card_for_user_in_status(
        &self,
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<PassthroughCard, DataError> {
        PassthroughCard::find_card_for_user_in_status(user_id, status).await
    }

}