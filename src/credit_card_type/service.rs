use std::sync::Arc;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;
use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
use crate::credit_card_type::error::CreditCardTypeError;
use crate::credit_card_type::model::{CreditCardDetailModel, CreditCardModel};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CreditCardServiceTrait {
    async fn list_all_card_types(self: Arc<Self>) -> Result<Vec<CreditCardDetailModel>, CreditCardTypeError>;
    async fn search_all_card_types(self: Arc<Self>, query: &str) -> Result<Vec<CreditCardDetailModel>, CreditCardTypeError>;
    async fn find_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<CreditCardModel, CreditCardTypeError>;
}

pub struct CreditCardService {
    credit_card_dao: Arc<dyn CreditCardDaoTrait>
}

impl CreditCardService {
    pub fn new() -> Self {
        Self {
            credit_card_dao: Arc::new(CreditCardDao::new())
        }
    }
    pub(super) fn new_with_services(
        credit_card_dao: Arc<dyn CreditCardDaoTrait>
    ) -> Self {
        Self {
            credit_card_dao: credit_card_dao.clone()
        }
    }
}

#[async_trait(?Send)]
impl CreditCardServiceTrait for CreditCardService {
    #[tracing::instrument(skip(self))]
    async fn list_all_card_types(self: Arc<Self>) -> Result<Vec<CreditCardDetailModel>, CreditCardTypeError> {
        tracing::info!("Listing all credit card");
        let cards = self.credit_card_dao.clone().list_all_card_types()
            .await.map_err(|e| {
            tracing::error!("Error listing all credit card types");
            CreditCardTypeError::Unexpected(e.into())
        })?;
        tracing::info!("Found {} credit cards", cards.len());
        let fin_cards: Vec<CreditCardDetailModel>  = cards.into_iter().map(|e| e.into()).collect();
        Ok(fin_cards)
    }

    #[tracing::instrument(skip(self))]
    async fn search_all_card_types(self: Arc<Self>, query: &str) -> Result<Vec<CreditCardDetailModel>, CreditCardTypeError> {
        tracing::info!("Searching for credit cards by query={}", &query);
        let cards = self.credit_card_dao.clone().search_all_card_types(query)
            .await.map_err(|e| {
            tracing::error!("Error searching all credit cards by query={}", &query);
            CreditCardTypeError::Unexpected(e.into())
        })?;
        tracing::info!("Found {} credit cards", cards.len());
        Ok(cards.into_iter().map(|e| e.into()).collect())
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<CreditCardModel, CreditCardTypeError> {
        tracing::info!("Searching for credit card by public_id={}", &public_id);
        let card = self.credit_card_dao.clone().find_by_public_id(public_id)
            .await.map_err(|e| {
            tracing::error!("Unexpected error finding card by public_id={}", &public_id);
            CreditCardTypeError::Unexpected(e.into())
        })?;
        tracing::info!("Found credit card id={}", &card.id);
        Ok(card.into())
    }
}