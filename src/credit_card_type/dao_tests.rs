#[cfg(test)]
mod test {
    use std::sync::Arc;
    use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
    use actix_web::test;

    #[test]
    pub async fn test_list() {
        crate::test_helper::general::init();
        let dao = Arc::new(CreditCardDao::new());
        let cards = dao.list_all_card_types().await.expect("Ok");
        assert_eq!(3, cards.len())
    }
}