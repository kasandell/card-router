use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::data_error::DataError;
use crate::user::entity::User;
use crate::wallet::entity::{InsertableCardAttempt, NewCard, Wallet, WalletCardAttempt, UpdateCardAttempt};
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait WalletDaoTrait {
    fn find_all_for_user(&self, user: &User) -> Result<Vec<Wallet>, DataError>;
    fn find_all_for_user_with_card_info(&self, user: &User) -> Result<Vec<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    fn insert_card(&self, card: NewCard) -> Result<Wallet, DataError>;
}


#[cfg_attr(test, automock)]
pub trait WalletCardAttemtDaoTrait {
    fn insert(&self, card_attempt: InsertableCardAttempt) -> Result<WalletCardAttempt, DataError>;
    fn find_by_reference_id(&self, reference: String) -> Result<WalletCardAttempt, DataError>;
    fn update_card(&self, id: i32, card: UpdateCardAttempt) -> Result<WalletCardAttempt, DataError>;


}

pub struct WalletDao {}
pub struct WalletCardAttemptDao {}

impl WalletDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl WalletDaoTrait for WalletDao {
    fn find_all_for_user(&self, user: &User) -> Result<Vec<Wallet>, DataError> {
        Wallet::find_all_for_user(user)
    }

    fn find_all_for_user_with_card_info(&self, user: &User) -> Result<Vec<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        Wallet::find_all_for_user_with_card_info(user)
    }

    fn insert_card(&self, card: NewCard) -> Result<Wallet, DataError> {
        Wallet::insert_card(card)
    }
}

impl WalletCardAttemptDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl WalletCardAttemtDaoTrait for WalletCardAttemptDao {
    fn insert(&self, card_attempt: InsertableCardAttempt) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::insert(card_attempt)
    }

    fn find_by_reference_id(&self, reference: String) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::find_by_reference_id(reference)
    }

    fn update_card(&self, id: i32, card: UpdateCardAttempt) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::update_card(id, card)
    }
}

