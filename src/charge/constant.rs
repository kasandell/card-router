use std::{fmt, io};
use std::io::Write;
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use diesel::backend::Backend;
use diesel::deserialize::{FromSql};
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::sql_types::*;
use std::fmt::{Display, Formatter};
use adyen_checkout::models::payment_response::ResultCode;
use serde::{Serializer};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum ChargeStatus {
    Fail,
    Success
}

impl ToSql<Text, Pg> for ChargeStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for ChargeStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"FAIL" => Ok(ChargeStatus::Fail),
            b"SUCCESS" => Ok(ChargeStatus::Success),
            v => Err(format!("Unknown value for ChargeStatus found").into()),

        }
    }
}

impl fmt::Display for ChargeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ChargeStatus::Fail => "FAIL",
            ChargeStatus::Success => "SUCCESS"
        })
    }
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





#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ChargeEngineResult {
    Approved,
    Denied,
    InsufficientFunds,
    CardClosed,
    CardPaused
}

impl ChargeEngineResult {
    pub fn is_success(self: &Self) -> Option<bool> {
        match *self {
            ChargeEngineResult::Approved => Some(true),
            _ => None
        }
    }
}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ChargeCardAttemptResult {
    Approved,
    Denied,
    PartialCancelSucceeded,
    PartialCancelFailed
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