use std::sync::Arc;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::error::data_error::DataError;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
use uuid::Uuid;

#[async_trait(?Send)]
pub trait CreditCardDaoTrait {
    async fn list_all_card_types(self: Arc<Self>) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError>;
    async fn find_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<CreditCard, DataError>;
}

pub struct CreditCardDao {}


impl CreditCardDao {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait(?Send)]
impl CreditCardDaoTrait for CreditCardDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn list_all_card_types(self: Arc<Self>) -> Result<Vec<(CreditCard, CreditCardType, CreditCardIssuer)>, DataError> {
        CreditCard::list_all_card_types().await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn find_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<CreditCard, DataError> {
        CreditCard::find_by_public_id(public_id).await
    }
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;
    use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
    use actix_web::test;
    use uuid::Uuid;
    use crate::error::data_error::DataError;

    #[test]
    async fn test_list() {
        crate::test_helper::general::init();
        let dao = Arc::new(CreditCardDao::new());
        let cards = dao.list_all_card_types().await.expect("Ok");
        assert_eq!(3, cards.len())
    }

    #[test]
    async fn test_find_by_pub_id_finds() {
        crate::test_helper::general::init();
        // This is a pub id pulled from db. if remigrate db, need ot grab this
        let dao = Arc::new(CreditCardDao::new());
        let cards = dao.clone().list_all_card_types().await.expect("OK");
        let id = cards[0].0.public_id;
        let card = dao.clone().find_by_public_id(&id).await.expect("ok");
        assert_eq!(card.id, cards[0].0.id);
        assert_eq!(card.public_id, id);
        assert_eq!(card.name, cards[0].0.name);
    }

    #[test]
    async fn test_find_by_pub_id_does_not_find() {
        crate::test_helper::general::init();
        let id = Uuid::new_v4();
        let dao = Arc::new(CreditCardDao::new());
        let error = dao.clone().find_by_public_id(&id).await.expect_err("ok");
        assert_eq!(DataError::NotFound("test".into()), error);
    }
}