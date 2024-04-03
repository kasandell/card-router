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
    async fn find_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<CreditCardModel, CreditCardTypeError>;
}

pub struct CreditCardService {
    credit_card_dao: Arc<dyn CreditCardDaoTrait>
}

impl CreditCardService {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        Self {
            credit_card_dao: Arc::new(CreditCardDao::new())
        }
    }
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
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


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;
    use actix_web::test;
    use uuid::Uuid;
    use crate::credit_card_type::dao::CreditCardDao;
    use crate::credit_card_type::error::CreditCardTypeError;
    use crate::credit_card_type::service::{CreditCardService, CreditCardServiceTrait};
    use crate::error::data_error::DataError;

    #[test]
    async fn test_list() {
        crate::test_helper::general::init();
        let svc = Arc::new(CreditCardService::new());
        let cards = svc.clone().list_all_card_types().await.expect("Ok");
        assert_eq!(3, cards.len())
    }

    #[test]
    async fn test_find_by_pub_id_finds() {
        crate::test_helper::general::init();
        // This is a pub id pulled from db. if remigrate db, need ot grab this
        let svc = Arc::new(CreditCardService::new());
        let cards = svc.clone().list_all_card_types().await.expect("OK");
        let id = cards[0].public_id;
        let card = svc.clone().find_by_public_id(&id).await.expect("ok");
        assert_eq!(card.id, cards[0].id);
        assert_eq!(card.public_id, id);
        assert_eq!(card.name, cards[0].name);
    }

    #[test]
    async fn test_find_by_pub_id_does_not_find() {
        crate::test_helper::general::init();
        let id = Uuid::new_v4();
        let svc = Arc::new(CreditCardService::new());
        let error = svc.clone().find_by_public_id(&id).await.expect_err("ok");
        assert_eq!(CreditCardTypeError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_new_with_services() {
        crate::test_helper::general::init();
        let dao = Arc::new(CreditCardDao::new());
        let svc = Arc::new(CreditCardService::new_with_services(dao));
    }
}