#[derive(Clone, Debug)]
pub enum CreditCardTypeEnum {
    ChaseSapphirePreferred = 1,
    ChaseSapphireReserve = 2,
    BiltWorldElite = 3
}

impl From<CreditCardTypeEnum> for i32 {
    fn from(value: CreditCardTypeEnum) -> Self {
        match value {
            CreditCardTypeEnum::ChaseSapphirePreferred => 1,
            CreditCardTypeEnum::ChaseSapphireReserve => 2,
            CreditCardTypeEnum::BiltWorldElite => 3
        }
    }
}