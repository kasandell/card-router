#[cfg(test)]
mod test {
    use crate::credit_card_type::entity::CreditCard;
    use actix_web::test;

    #[test]
    pub async fn test_list() {
        crate::test_helper::general::init();
        let cards = CreditCard::list_all_card_types().await.expect("Ok");
        assert_eq!(3, cards.len())
    }
}