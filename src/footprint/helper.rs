use std::ops::Add;
use footprint::models::CreateClientTokenRequest;
use crate::footprint::r#enum::CardPart;
use crate::footprint::constant::Constant::TTL;

pub fn card_request_parts_for_card_id(card_id: &str) -> Vec<String> {
    // given card, return
    /*
            "card.{card_id}.number",
            "card.{card_id}.cvc",
            "card.{card_id}.expiration",
            "card.{card_id}.name"
     */
    CardPart::all_parts()
        .iter()
        .map(|part| individual_request_part(card_id, part))
        .collect()
}

pub fn individual_request_part(card_id: &str, part: &CardPart) -> String {
    // card.XXXXX.expiry
    return "card.".to_string().add(card_id).add(".").add(part.as_str());
}

pub fn individual_request_part_for_customer(customer_id: &str, card_id: &str, part: &CardPart) -> String {
    // CCCC.card.XXXXX.expiry
    return customer_id.to_string().add(".card.").add(card_id).add(".").add(part.as_str());
}

pub fn individual_request_part_for_customer_template(customer_id: &str, card_id: &str, part: &CardPart) -> String {
    // {{ CCCC.card.XXXXX.expiry }}
    return "{{ ".to_string().add(&customer_id).add(".card.").add(card_id).add(".").add(part.as_str()).add(" }}");
}

pub fn individual_request_part_for_customer_with_prefix_template(customer_id: &str, card_id: &str, part: &CardPart) -> String {
    // {{ CCCC.card.XXXXX.expiry | prefix(2) }} used for extracting month
    return "{{ ".to_string().add(&customer_id).add(".card.").add(card_id).add(".").add(part.as_str()).add(" | prefix(2) }}");
}

pub fn individual_request_part_for_customer_with_suffix_template(customer_id: &str, card_id: &str, part: &CardPart) -> String {
    // {{ CCCC.card.XXXXX.expiry | suffix(2) }} used for extracting year
    return "{{ ".to_string().add(&customer_id).add(".card.").add(card_id).add(".").add(part.as_str()).add(" | suffix(4) }}");
}

pub fn get_scopes_for_request() -> Vec<String> {
    vec!["vault".to_string()]
}

// TODO: this might need to have user id somewhere
pub fn create_get_token_request(customer_id: &str, card_id: &str) -> CreateClientTokenRequest {
    CreateClientTokenRequest {
        ttl: TTL,
        scopes: get_scopes_for_request(),
        fields: card_request_parts_for_card_id(card_id),
    }
}


#[cfg(test)]
mod helper_tests {
    use crate::footprint::r#enum::CardPart;
    use actix_web;
    use crate::footprint::constant::Constant::TTL;
    use crate::footprint::helper::{card_request_parts_for_card_id, individual_request_part, individual_request_part_for_customer_with_prefix_template, individual_request_part_for_customer_with_suffix_template, individual_request_part_for_customer_template, individual_request_part_for_customer, get_scopes_for_request, create_get_token_request};

    #[test]
    fn test_get_scopes_for_request() {
        assert_eq!(vec!["vault".to_string()], get_scopes_for_request());
    }

    #[test]
    fn test_create_get_token_request() {
        let customer_id = "1234";
        let card_id = "5678";
        let req = create_get_token_request(customer_id, card_id);
        assert_eq!(TTL, req.ttl);
        assert_eq!(vec!["vault".to_string()], req.scopes);
        assert_eq!(vec![
            "card.5678.cvc".to_string(),
            "card.5678.name".to_string(),
            "card.5678.expiration".to_string(),
            "card.5678.number".to_string()
        ], req.fields);
    }

    #[test]
    fn test_individual_request_part() {
        let card_id = "1234";
        assert_eq!("card.1234.number", &individual_request_part(card_id, &CardPart::CardNumber));
        assert_eq!("card.1234.cvc", &individual_request_part(card_id, &CardPart::Cvc));
        assert_eq!("card.1234.expiration", &individual_request_part(card_id, &CardPart::Expiration));
        assert_eq!("card.1234.name", &individual_request_part(card_id, &CardPart::Name));
    }

    #[test]
    fn test_all_request_parts() {
        let card_id = "1234";
        let expected = vec![
            "card.1234.cvc".to_string(),
            "card.1234.name".to_string(),
            "card.1234.expiration".to_string(),
            "card.1234.number".to_string()
        ];
        let parts = card_request_parts_for_card_id(card_id);
        assert_eq!(expected, parts);
    }


    #[test]
    fn test_individual_request_part_for_customer() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("abc.card.1234.number", &individual_request_part_for_customer(customer_id, card_id, &CardPart::CardNumber));
        assert_eq!("abc.card.1234.cvc", &individual_request_part_for_customer(customer_id, card_id, &CardPart::Cvc));
        assert_eq!("abc.card.1234.expiration", &individual_request_part_for_customer(customer_id, card_id, &CardPart::Expiration));
        assert_eq!("abc.card.1234.name", &individual_request_part_for_customer(customer_id, card_id, &CardPart::Name));
    }

    #[test]
    fn test_individual_request_part_for_customer_template() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("{{ abc.card.1234.number }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::CardNumber));
        assert_eq!("{{ abc.card.1234.cvc }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::Cvc));
        assert_eq!("{{ abc.card.1234.expiration }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::Expiration));
        assert_eq!("{{ abc.card.1234.name }}", &individual_request_part_for_customer_template(customer_id, card_id, &CardPart::Name));
    }

    #[test]
    fn test_individual_request_part_for_customer_template_suffix() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("{{ abc.card.1234.expiration | suffix(4) }}", &individual_request_part_for_customer_with_suffix_template(customer_id, card_id, &CardPart::Expiration));
    }

    #[test]
    fn test_individual_request_part_for_customer_template_prefix() {
        let card_id = "1234";
        let customer_id = "abc";
        assert_eq!("{{ abc.card.1234.expiration | prefix(2) }}", &individual_request_part_for_customer_with_prefix_template(customer_id, card_id, &CardPart::Expiration));
    }
}