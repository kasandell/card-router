use std::ops::Add;
use footprint::models::CreateClientTokenRequest;
use crate::footprint::r#enum::CardPart;
use crate::error::service_error::ServiceError;
use crate::footprint::constant::Constant::TTL;

pub fn card_request_parts_for_card_id(card_id: &str) -> Result<Vec<String>, ServiceError> {
    // given card, return
    /*
            "card.{card_id}.number",
            "card.{card_id}.cvc",
            "card.{card_id}.expiration",
            "card.{card_id}.name"
     */
    Ok(
        CardPart::all_parts()
            .iter()
            .map(|part| individual_request_part(card_id, part))
            .collect()
    )
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
    return "{{ ".to_string().add(&customer_id).add(".card.").add(card_id).add(".").add(part.as_str()).add(" | suffix(2) }}");
}

pub fn get_scopes_for_request() -> Vec<String> {
    vec!["vault".to_string()]
}

pub fn create_get_token_request(customer_id: &str, card_id: &str) -> Result<CreateClientTokenRequest, ServiceError> {
    Ok(CreateClientTokenRequest {
        ttl: TTL,
        scopes: get_scopes_for_request(),
        fields: card_request_parts_for_card_id(card_id)?,
    })
}