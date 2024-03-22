use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::credit_card_type::typedef::CreditCardDetail;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditCardModel {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub credit_card_type_id: i32,
    pub credit_card_issuer_id: i32,
    pub card_image_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditCardDetailModel {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub credit_card_type_id: i32,
    pub credit_card_issuer_id: i32,
    pub card_image_url: String,
    pub credit_card_type_name: String,
    pub credit_card_issuer_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditCardTypeModel {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditCardIssuerModel {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
}


impl From<CreditCard> for CreditCardModel {
    fn from(value: CreditCard) -> Self {
        CreditCardModel {
            id: value.id,
            public_id: value.public_id,
            name: value.name,
            credit_card_type_id: value.credit_card_type_id,
            credit_card_issuer_id: value.credit_card_issuer_id,
            card_image_url: value.card_image_url,
        }
    }
}

impl From<CreditCardType> for CreditCardTypeModel {
    fn from(value: CreditCardType) -> Self {
        CreditCardTypeModel {
            id: value.id,
            public_id: value.public_id,
            name: value.name,
        }
    }
}

impl From<CreditCardDetail> for CreditCardDetailModel {
    fn from(value: CreditCardDetail) -> Self {
        CreditCardDetailModel {
            id: value.0.id,
            public_id: value.0.public_id,
            name: value.0.name,
            credit_card_type_id: value.0.credit_card_type_id,
            credit_card_issuer_id: value.0.credit_card_issuer_id,
            card_image_url: value.0.card_image_url,
            credit_card_type_name: value.1.name,
            credit_card_issuer_name: value.2.name
        }
    }
}

