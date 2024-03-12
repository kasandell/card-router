use crate::rule::constant::DayOfMonth;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChargeStatus {
    Fail,
    Success
}

impl ChargeStatus {
    pub fn as_str(&self) -> String {
        match self {
            ChargeStatus::Fail => "FAIL".to_string(),
            ChargeStatus::Success => "SUCCESS".to_string(),
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "FAIL" => ChargeStatus::Fail,
            "SUCCESS" => ChargeStatus::Success,
            _ => ChargeStatus::Fail
        }
    }
}