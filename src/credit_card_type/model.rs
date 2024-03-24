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

impl From<CreditCardIssuer> for CreditCardIssuerModel{
    fn from(value: CreditCardIssuer) -> Self {
        CreditCardIssuerModel {
            id: value.id,
            public_id: value.public_id,
            name: value.name
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



#[cfg(test)]
mod test {
    use uuid::Uuid;
    use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
    use crate::credit_card_type::model::{CreditCardIssuerModel, CreditCardModel, CreditCardTypeModel};
    use crate::credit_card_type::typedef::CreditCardDetail;

    const CREDIT_CARD_NAME: &str = "Sapphire Reserve";
    const CREDIT_CARD_ISSUER: &str = "Chase";
    const CARD_IMAGE_URL: &str = "www.prettyphoto.com";

    #[test]
    pub fn test_from_credit_card_detail() {

    }

    #[test]
    pub fn test_from_credit_card() {
        let public_id = Uuid::new_v4();
        let card = CreditCard {
            id: 3,
            public_id: public_id.clone(),
            name: CREDIT_CARD_NAME.to_string(),
            credit_card_type_id: 2,
            credit_card_issuer_id: 5,
            card_image_url: CARD_IMAGE_URL.to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        let model = CreditCardModel::from(card.clone());
        assert_eq!(model.id, card.id);
        assert_eq!(model.public_id, card.public_id);
        assert_eq!(model.card_image_url, card.card_image_url);
        assert_eq!(model.name, card.name);
        assert_eq!(model.credit_card_issuer_id, card.credit_card_issuer_id);
        assert_eq!(model.credit_card_type_id, card.credit_card_type_id);
    }

    #[test]
    pub fn test_from_credit_card_issuer() {

        let issuer = CreditCardIssuer {
            id: 1,
            public_id: Uuid::new_v4(),
            name: CREDIT_CARD_ISSUER.to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let model = CreditCardIssuerModel::from(issuer.clone());

        assert_eq!(model.id, issuer.id);
        assert_eq!(model.public_id, issuer.public_id);
        assert_eq!(model.name, issuer.name);
    }

    #[test]
    pub fn test_from_credit_card_type() {
        let cc_type = CreditCardType {
            id: 2,
            public_id: Uuid::new_v4(),
            name: CREDIT_CARD_NAME.to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let model = CreditCardTypeModel::from(cc_type.clone());

        assert_eq!(model.id, cc_type.id);
        assert_eq!(model.public_id, cc_type.public_id);
        assert_eq!(model.name, cc_type.name);
    }
}
