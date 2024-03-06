use std::sync::Arc;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::data_error::DataError;
use crate::user::entity::User;
use crate::wallet::entity::{InsertableCardAttempt, NewCard, Wallet, WalletCardAttempt, UpdateCardAttempt};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WalletDaoTrait {
    async fn find_all_for_user(self: Arc<Self>, user: &User) -> Result<Vec<Wallet>, DataError>;
    async fn find_all_for_user_with_card_info(self: Arc<Self>, user: &User) -> Result<Vec<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    async fn insert_card<'a>(self: Arc<Self>, card: &NewCard<'a>) -> Result<Wallet, DataError>;
}


#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WalletCardAttemtDaoTrait {
    async fn insert<'a>(self: Arc<Self>, card_attempt: &InsertableCardAttempt<'a>) -> Result<WalletCardAttempt, DataError>;
    async fn find_by_reference_id(self: Arc<Self>, reference: &str) -> Result<WalletCardAttempt, DataError>;
    async fn update_card<'a>(self: Arc<Self>, id: i32, card: &UpdateCardAttempt<'a>) -> Result<WalletCardAttempt, DataError>;


}

pub struct WalletDao {}
pub struct WalletCardAttemptDao {}

impl WalletDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl WalletDaoTrait for WalletDao {
    async fn find_all_for_user(self: Arc<Self>, user: &User) -> Result<Vec<Wallet>, DataError> {
        Wallet::find_all_for_user(user).await
    }

    async fn find_all_for_user_with_card_info(self: Arc<Self>, user: &User) -> Result<Vec<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        Wallet::find_all_for_user_with_card_info(user).await
    }

    async fn insert_card<'a>(self: Arc<Self>, card: &NewCard<'a>) -> Result<Wallet, DataError> {
        Wallet::insert_card(card).await
    }
}

impl WalletCardAttemptDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl WalletCardAttemtDaoTrait for WalletCardAttemptDao {
    async fn insert<'a>(self: Arc<Self>, card_attempt: &InsertableCardAttempt<'a>) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::insert(card_attempt).await
    }

    async fn find_by_reference_id(self: Arc<Self>, reference: &str) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::find_by_reference_id(reference).await
    }

    async fn update_card<'a>(self: Arc<Self>, id: i32, card: &UpdateCardAttempt<'a>) -> Result<WalletCardAttempt, DataError> {
        WalletCardAttempt::update_card(id, card).await
    }
}

