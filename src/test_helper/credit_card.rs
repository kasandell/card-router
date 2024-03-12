use chrono::Utc;
use uuid::Uuid;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
pub fn create_mock_credit_card(
    name: &str,
) -> CreditCard {
    CreditCard {
        id: 1,
        public_id: Uuid::new_v4(),
        name: name.to_string(),
        credit_card_type_id: 1,
        credit_card_issuer_id: 1,
        card_image_url: "".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_credit_card_with_args(
    id: i32,
    name: &str,
    credit_card_type_id: i32,
    credit_card_issuer_id: i32
) -> CreditCard {
    CreditCard {
        id: id,
        public_id: Uuid::new_v4(),
        name: name.to_string(),
        credit_card_type_id: credit_card_type_id,
        credit_card_issuer_id: credit_card_issuer_id,
        card_image_url: "".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_credit_card_issuer(name: &str) -> CreditCardIssuer {
    CreditCardIssuer {
        id: 1,
        public_id: Uuid::new_v4(),
        name: name.to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_credit_card_issuer_with_args(
    id: i32,
    name: &str,
) -> CreditCardIssuer {
    CreditCardIssuer {
        id: id,
        public_id: Uuid::new_v4(),
        name: name.to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_credit_card_type(name: &str) -> CreditCardType {
    CreditCardType {
        id: 1,
        public_id: Uuid::new_v4(),
        name: name.to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_credit_card_type_with_args(id: i32, name: &str) -> CreditCardType {
    CreditCardType {
        id: id,
        public_id: Uuid::new_v4(),
        name: name.to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}
