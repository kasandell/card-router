use std::ops::Add;
use crate::footprint_service::r#enum::CardPart;
use crate::service_error::ServiceError;

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
    return "card.".to_string().add(card_id).add(".").add(part.as_str());
}

pub fn individual_request_part_for_customer(customer_id: &str, card_id: &str, part: &CardPart) -> String {
    return customer_id.to_string().add("card.").add(card_id).add(".").add(part.as_str());
}