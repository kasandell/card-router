use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::data_error::DataError;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CreditCardDaoTrait {
    async fn list_all_card_types(&self) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    async fn search_all_card_types(&self, query: String) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    async fn find_by_public_id(&self, public_id: Uuid) -> Result<CreditCard, DataError>;
}

pub struct CreditCardDao {}


impl CreditCardDao {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl CreditCardDaoTrait for CreditCardDao {
    async fn list_all_card_types(&self) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        CreditCard::list_all_card_types().await
    }

    async fn search_all_card_types(&self, query: String) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        CreditCard::search_all_card_types(query).await
    }
    async fn find_by_public_id(&self, public_id: Uuid) -> Result<CreditCard, DataError> {
        CreditCard::find_by_public_id(public_id).await
    }
}