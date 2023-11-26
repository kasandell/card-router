#[derive(Debug)]
pub enum PassthroughCardType {
    VIRTUAL,
    PHYSICAL,
    MERCHANT_LOCKED,
    SINGLE_USE
}

impl PassthroughCardType {
    fn as_str(&self) -> String {
        match self {
            PassthroughCardType::VIRTUAL => "VIRTUAL",
            PassthroughCardType::PHYSICAL => "PHYSICAL",
            PassthroughCardType::MERCHANT_LOCKED => "MERCHANT_LOCKED",
            PassthroughCardType::SINGLE_USE => "SINGLE_USE",
        }
    }
}

#[derive(Debug)]
pub enum PassthroughCardStatus {
    CLOSED,
    OPEN,
    PAUSED,
    PENDING_ACTIVATION,
    PENDING_FULFILLMENT
}

impl PassthroughCardStatus {
    fn as_str(&self) -> String {
        match self {
            PassThroughCardStatus::CLOSED => "CLOSED",
            PassThroughCardStatus::OPEN => "OPEN",
            PassThroughCardStatus::PAUSED => "PAUSED",
            PassThroughCardStatus::PENDING_ACTIVATION => "PENDING_ACTIVATION",
            PassThroughCardStatus::PENDING_FULFILLMENT => "PENDING_FULFILLMENT",
        }
    }
}
