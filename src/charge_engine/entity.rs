use std::fmt::{Display, Formatter};
use adyen_checkout::models::payment_response::ResultCode;
use serde::{Deserialize, Serialize, Serializer};
use serde::de::Unexpected::Char;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ChargeEngineResult {
    Approved,
    Denied,
    InsufficientFunds,
    CardClosed,
    CardPaused
}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ChargeCardAttemptResult {
    Approved,
    Denied,
    PartialCancelSucceeded,
    PartialCancelFailed
}

impl Display for ChargeEngineResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_str(&self.to_string())
    }
}

impl From<&ChargeCardAttemptResult> for bool {
    fn from(value: &ChargeCardAttemptResult) -> Self {
        match *value {
            ChargeCardAttemptResult::Approved => true,
            _ => false
        }
    }
}

impl From<ResultCode> for ChargeCardAttemptResult {
    fn from(value: ResultCode) -> Self {
        match value {
            ResultCode::Authorised => ChargeCardAttemptResult::Approved,
            ResultCode::Pending => ChargeCardAttemptResult::Approved,
            ResultCode::Received => ChargeCardAttemptResult::Approved,
            ResultCode::Success => ChargeCardAttemptResult::Approved,

            ResultCode::AuthenticationFinished => ChargeCardAttemptResult::Denied,
            ResultCode::AuthenticationNotRequired => ChargeCardAttemptResult::Denied,
            ResultCode::Cancelled => ChargeCardAttemptResult::Denied,
            ResultCode::ChallengeShopper => ChargeCardAttemptResult::Denied,
            ResultCode::Error => ChargeCardAttemptResult::Denied,
            ResultCode::IdentifyShopper => ChargeCardAttemptResult::Denied,
            ResultCode::PartiallyAuthorised => ChargeCardAttemptResult::Denied,
            ResultCode::PresentToShopper => ChargeCardAttemptResult::Denied,
            ResultCode::RedirectShopper => ChargeCardAttemptResult::Denied,
            ResultCode::Refused => ChargeCardAttemptResult::Denied,
        }
    }
}