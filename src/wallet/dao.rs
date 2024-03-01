use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::data_error::DataError;
use crate::user::entity::User;
use crate::wallet::entity::{InsertableCardAttempt, NewCard, Wallet, WalletCardAttempt, UpdateCardAttempt};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WalletDaoTrait {
    async fn find_all_for_user(&self, user: &User) -> Result<Vec<Wallet>, DataError>;
    async fn find_all_for_user_with_card_info(&self, user: &User) -> Result<Vec<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    async fn insert_card(&self, card: NewCard) -> Result<Wallet, DataError>;
}


#[cfg_attr(test, automock)]
#[async_trait]
pub trait WalletCardAttemtDaoTrait {
    async fn insert(&self, card_attempt: InsertableCardAttempt) -> Result<WalletCardAttempt, DataError>;
    async fn find_by_reference_id(&self, reference: String) -> Result<WalletCardAttempt, DataError>;
    async fn update_card(&self, id: i32, card: UpdateCardAttempt) -> Result<WalletCardAttempt, DataError>;


}

pub struct WalletDao {}
pub struct WalletCardAttemptDao {}

impl WalletDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl WalletDaoTrait for WalletDao {
    async fn find_all_for_user(&self, user: &User) -> Result<Vec<Wallet>, DataError> {
        Wallet::find_all_for_user(user).await
    }

    async fn find_all_for_user_with_card_info(&self, user: &User) -> Result<Vec<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        Wallet::find_all_for_user_with_card_info(user).await
    }

    async fn insert_card(&self, card: NewCard) -> Result<Wallet, DataError> {
        Wallet::insert_card(card).await
    }
}

impl WalletCardAttemptDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl WalletCardAttemtDaoTrait for WalletCardAttemptDao {
    async fn insert(&self, card_attempt: InsertableCardAttempt) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::insert(card_attempt).await
    }

    async fn find_by_reference_id(&self, reference: String) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::find_by_reference_id(reference).await
    }

    async fn update_card(&self, id: i32, card: UpdateCardAttempt) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::update_card(id, card).await
    }
}

