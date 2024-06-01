use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::charge::constant::ChargeEngineResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AsaResponseResult {
    AccountInactive,
    AvsInvalid,
    CardClosed,
    CardPaused,
    InsufficientFunds,
    UnauthorizedMerchant,
    VelocityExceeded,
    Approved
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AvsResponseResult {
    Fail,
    Match,
    MatchAddressOnly,
    MatchZipOnly
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AsaResponse {
    pub token: String,
    pub result: AsaResponseResult,
    pub avs_result: Option<String>,
    pub balance: Option<Balance>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Balance {
    pub amount: i32,
    pub available: i32
}

impl Serialize for AsaResponseResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl<'de> Deserialize<'de> for AsaResponseResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(AsaResponseResult::from(s))
    }
}

impl From<&str> for AsaResponseResult {
    fn from(value: &str) -> Self {
        match value {
            "ACCOUNT_INACTIVE" => AsaResponseResult::AccountInactive,
            "AVS_INVALID" => AsaResponseResult::AvsInvalid,
            "CARD_CLOSED" => AsaResponseResult::CardClosed,
            "CARD_PAUSED" => AsaResponseResult::CardPaused,
            "INSUFFICIENT_FUNDS" => AsaResponseResult::InsufficientFunds,
            "UNAUTHORIZED_MERCHANT" => AsaResponseResult::UnauthorizedMerchant,
            "VELOCITY_EXCEEDED" => AsaResponseResult::VelocityExceeded,
            "APPROVED" => AsaResponseResult::Approved,
            _ => AsaResponseResult::UnauthorizedMerchant
        }
    }
}

impl From<String> for AsaResponseResult {
    fn from(value: String) -> Self {
        AsaResponseResult::from(&*value)
    }
}

impl From<AsaResponseResult> for String {
    fn from(value: AsaResponseResult) -> Self {
        String::from(&value)
    }
}

impl From<&AsaResponseResult> for String {
    fn from(value: &AsaResponseResult) -> Self {
        match *value {
            AsaResponseResult::AccountInactive => "ACCOUNT_INACTIVE",
            AsaResponseResult::AvsInvalid => "AVS_INVALID",
            AsaResponseResult::CardClosed => "CARD_CLOSED",
            AsaResponseResult::CardPaused => "CARD_PAUSED",
            AsaResponseResult::InsufficientFunds => "INSUFFICIENT_FUNDS",
            AsaResponseResult::UnauthorizedMerchant => "UNAUTHORIZED_MERCHANT",
            AsaResponseResult::VelocityExceeded => "VELOCITY_EXCEEDED",
            AsaResponseResult::Approved => "APPROVED",
        }.to_string()
    }
}

impl From<ChargeEngineResult> for AsaResponseResult {
    fn from(value: ChargeEngineResult) -> Self {
        match value {
            ChargeEngineResult::Approved => AsaResponseResult::Approved,
            ChargeEngineResult::CardClosed => AsaResponseResult::CardClosed,
            ChargeEngineResult::Denied => AsaResponseResult::UnauthorizedMerchant,
            ChargeEngineResult::InsufficientFunds => AsaResponseResult::InsufficientFunds,
            ChargeEngineResult::CardPaused => AsaResponseResult::CardPaused,
            _ => AsaResponseResult::AccountInactive
        }
    }
}

impl Serialize for AvsResponseResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl<'de> Deserialize<'de> for AvsResponseResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(AvsResponseResult::from(s))
    }
}

impl From<String> for AvsResponseResult {
    fn from(value: String) -> Self {
        AvsResponseResult::from(value.as_str())
    }
}

impl From<&str> for AvsResponseResult {
    fn from(value: &str) -> Self {
        match value {
            "FAIL" => AvsResponseResult::Fail,
            "MATCH" => AvsResponseResult::Match,
            "MATCH_ADDRESS_ONLY" => AvsResponseResult::MatchAddressOnly,
            "MATCH_ZIP_ONLY" => AvsResponseResult::MatchZipOnly,
            _ => AvsResponseResult::Fail
        }
    }
}

impl From<AvsResponseResult> for String {
    fn from(value: AvsResponseResult) -> Self {
        String::from(&value)
    }
}

impl From<&AvsResponseResult> for String {
    fn from(value: &AvsResponseResult) -> Self {
        match *value {
            AvsResponseResult::Fail => "FAIL",
            AvsResponseResult::Match => "MATCH",
            AvsResponseResult::MatchAddressOnly => "MATCH_ADDRESS_ONLY",
            AvsResponseResult::MatchZipOnly => "MATCH_ZIP_ONLY"
        }.to_string()
    }
}
