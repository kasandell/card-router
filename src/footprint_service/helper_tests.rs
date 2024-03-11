#[cfg(test)]
mod helper_tests {
    use crate::footprint_service::r#enum::CardPart;
    use actix_web;
    use crate::footprint_service::helper::{
        card_request_parts_for_card_id,
        individual_request_part,
        individual_request_part_for_customer_with_prefix_template,
        individual_request_part_for_customer_with_suffix_template,
        individual_request_part_for_customer_template,
        individual_request_part_for_customer
    };

    #[actix_web::test]
    async fn test_individual_request_part() {
        let card_id = "1234";
        assert_eq!("card.1234.number", &individual_request_part(card_id, &CardPart::CardNumber));
        assert_eq!("card.1234.cvc", &individual_request_part(card_id, &CardPart::Cvc));
        assert_eq!("card.1234.expiration", &individual_request_part(card_id, &CardPart::Expiration));
        assert_eq!("card.1234.name", &individual_request_part(card_id, &CardPart::Name));
    }

    #[actix_web::test]
    async fn test_all_request_parts() {
        let card_id = "1234";
        let expected = vec![
            "card.1234.cvc".to_string(),
            "card.1234.name".to_string(),
            "card.1234.expiration".to_string(),
            "card.1234.number".to_string()
        ];
        let parts = card_request_parts_for_card_id(card_id).expect("no error");
        assert_eq!(expected, parts);
    }


    #[actix_web::test]
    async fn test_individual_request_part_for_customer() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("abc.card.1234.number", &individual_request_part_for_customer(customer_id, card_id, &CardPart::CardNumber));
        assert_eq!("abc.card.1234.cvc", &individual_request_part_for_customer(customer_id, card_id, &CardPart::Cvc));
        assert_eq!("abc.card.1234.expiration", &individual_request_part_for_customer(customer_id, card_id, &CardPart::Expiration));
        assert_eq!("abc.card.1234.name", &individual_request_part_for_customer(customer_id, card_id, &CardPart::Name));
    }

    #[actix_web::test]
    async fn test_individual_request_part_for_customer_template() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("{{ abc.card.1234.number }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::CardNumber));
        assert_eq!("{{ abc.card.1234.cvc }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::Cvc));
        assert_eq!("{{ abc.card.1234.expiration }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::Expiration));
        assert_eq!("{{ abc.card.1234.name }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::Name));
    }

    #[actix_web::test]
    async fn test_individual_request_part_for_customer_template_suffix() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("{{ abc.card.1234.expiration | suffix(2) }}", &individual_request_part_for_customer_with_suffix_template(customer_id, card_id, &CardPart::Expiration));
    }

    #[actix_web::test]
    async fn test_individual_request_part_for_customer_template_prefix() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("{{ abc.card.1234.expiration | prefix(2) }}", &individual_request_part_for_customer_with_prefix_template(customer_id, card_id, &CardPart::Expiration));
    }
}