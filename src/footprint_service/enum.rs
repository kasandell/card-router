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