#[derive(Debug)]
pub enum PassthroughCardType {
    VIRTUAL,
    PHYSICAL,
    MERCHANT_LOCKED,
    SINGLE_USE
}

#[derive(Debug, Clone)]
pub enum PassthroughCardStatus {
    CLOSED,
    OPEN,
    PAUSED,
    PENDING_ACTIVATION,
    PENDING_FULFILLMENT
}

impl PassthroughCardStatus {
    pub fn is_active_for_status(&self) -> Option<bool> {
        match *self {
            PassthroughCardStatus::CLOSED => None,
            PassthroughCardStatus::OPEN => Some(true),
            PassthroughCardStatus::PAUSED => Some(true),
            PassthroughCardStatus::PENDING_ACTIVATION => Some(true),
            PassthroughCardStatus::PENDING_FULFILLMENT => Some(true),
        }
    }
}

impl From<&PassthroughCardType> for String {
    fn from(value: &PassthroughCardType) -> Self {
        match *value {
            PassthroughCardType::VIRTUAL => "VIRTUAL".to_string(),
            PassthroughCardType::PHYSICAL => "PHYSICAL".to_string(),
            PassthroughCardType::MERCHANT_LOCKED => "MERCHANT_LOCKED".to_string(),
            PassthroughCardType::SINGLE_USE => "SINGLE_USE".to_string(),
        }
    }
}

impl From<&str> for PassthroughCardType {
    fn from(value: &str) -> Self {
        match value {
            "VIRTUAL" => PassthroughCardType::VIRTUAL,
            "PHYSICAL" => PassthroughCardType::PHYSICAL,
            "MERCHANT_LOCKED" => PassthroughCardType::MERCHANT_LOCKED,
            "SINGLE_USE" => PassthroughCardType::SINGLE_USE,
            _ => PassthroughCardType::VIRTUAL
        }
    }
}


impl From<&PassthroughCardStatus> for String {
    fn from(value: &PassthroughCardStatus) -> Self {
        match *value {
            PassthroughCardStatus::CLOSED => "CLOSED".to_string(),
            PassthroughCardStatus::OPEN => "OPEN".to_string(),
            PassthroughCardStatus::PAUSED => "PAUSED".to_string(),
            PassthroughCardStatus::PENDING_ACTIVATION => "PENDING_ACTIVATION".to_string(),
            PassthroughCardStatus::PENDING_FULFILLMENT => "PENDING_FULFILLMENT".to_string(),
        }
    }
}

impl From<&str> for PassthroughCardStatus {
    fn from(value: &str) -> Self {
        match value {
            "CLOSED" => PassthroughCardStatus::CLOSED,
            "OPEN" => PassthroughCardStatus::OPEN,
            "PAUSED" => PassthroughCardStatus::PAUSED,
            "PENDING_ACTIVATION" => PassthroughCardStatus::PENDING_ACTIVATION,
            "PENDING_FULFILLMENT" => PassthroughCardStatus::PENDING_FULFILLMENT,
            _ => PassthroughCardStatus::PAUSED,
        }
    }
}