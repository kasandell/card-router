use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::data_error::DataError;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait CreditCardDaoTrait {
    fn list_all_card_types(&self) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    fn search_all_card_types(&self, query: String) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
}

pub struct CreditCardDao {}


impl CreditCardDao {
    pub fn new() -> Self {
        Self{}
    }
}

impl CreditCardDaoTrait for CreditCardDao {
    fn list_all_card_types(&self) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        CreditCard::list_all_card_types()
    }

    fn search_all_card_types(&self, query: String) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        CreditCard::search_all_card_types(query)
    }
}