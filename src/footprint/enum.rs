#[derive(PartialEq, Debug)]
pub enum CardPart {
    CardNumber,
    Cvc,
    Expiration,
    Name,
}

impl CardPart {

    pub fn as_str(&self) -> &str {
        return match self {
            CardPart::Cvc => "cvc",
            CardPart::Expiration => "expiration",
            CardPart::Name => "name",
            CardPart::CardNumber => "number"
        }
    }

    pub fn all_parts() -> Vec<Self> {
        vec![CardPart::Cvc, CardPart::Name, CardPart::Expiration, CardPart::CardNumber]
    }
}

#[cfg(test)]
mod enum_tests {
    use crate::footprint::r#enum::CardPart;
    use actix_web;

    #[test]
    fn test_individual_enum_items() {
        assert_eq!("cvc", CardPart::Cvc.as_str());
        assert_eq!("expiration", CardPart::Expiration.as_str());
        assert_eq!("name", CardPart::Name.as_str());
        assert_eq!("number", CardPart::CardNumber.as_str());
    }

    #[test]
    fn test_all_parts_list() {
        assert_eq!(
            vec![CardPart::Cvc, CardPart::Name, CardPart::Expiration, CardPart::CardNumber],
            CardPart::all_parts()
        );
    }
}