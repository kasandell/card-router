use std::{fmt, io};
use std::io::Write;
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output, IsNull};
use diesel::sql_types::*;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum PassthroughCardType {
    Virtual,
    Physical,
    MerchantLocked,
    SingleUse
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum PassthroughCardStatus {
    Closed,
    Open,
    Paused,
    PendingActivation,
    PendingFulfillment
}

impl ToSql<Text, Pg> for PassthroughCardType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for PassthroughCardType {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"VIRTUAL" => Ok(PassthroughCardType::Virtual),
            b"PHYSICAL" => Ok(PassthroughCardType::Physical),
            b"MERCHANT_LOCKED" => Ok(PassthroughCardType::MerchantLocked),
            b"SINGLE_USE" => Ok(PassthroughCardType::SingleUse),
            v => Err(format!("Unknown value for PassthroughCardType found").into()),

        }
    }
}

impl ToSql<Text, Pg> for PassthroughCardStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for PassthroughCardStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"CLOSED" => Ok(PassthroughCardStatus::Closed),
            b"OPEN" => Ok(PassthroughCardStatus::Open),
            b"PAUSED" => Ok(PassthroughCardStatus::Paused),
            b"PENDING_ACTIVATION" => Ok(PassthroughCardStatus::PendingActivation),
            b"PENDING_FULFILLMENT" => Ok(PassthroughCardStatus::PendingFulfillment),
            v => Err(format!("Unknown value for PassthroughCardStatus found").into())
        }
    }
}

impl PassthroughCardStatus {
    pub fn is_active_for_status(&self) -> Option<bool> {
        match *self {
            PassthroughCardStatus::Closed => None,
            PassthroughCardStatus::Open => Some(true),
            PassthroughCardStatus::Paused => Some(true),
            PassthroughCardStatus::PendingActivation => Some(true),
            PassthroughCardStatus::PendingFulfillment => Some(true),
        }
    }
}

impl fmt::Display for PassthroughCardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            PassthroughCardType::Virtual => "VIRTUAL",
            PassthroughCardType::Physical => "PHYSICAL",
            PassthroughCardType::MerchantLocked => "MERCHANT_LOCKED",
            PassthroughCardType::SingleUse => "SINGLE_USE"
        })
    }
}

impl fmt::Display for PassthroughCardStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            PassthroughCardStatus::Closed => "CLOSED",
            PassthroughCardStatus::Open => "OPEN",
            PassthroughCardStatus::Paused => "PAUSED",
            PassthroughCardStatus::PendingActivation => "PENDING_ACTIVATION",
            PassthroughCardStatus::PendingFulfillment => "PENDING_FULFILLMENT",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};

    #[test]
    fn test_conversion_passthrough_card_type() {
        assert_eq!(PassthroughCardType::Virtual.to_string(), "VIRTUAL".to_string());
        assert_eq!(PassthroughCardType::Physical.to_string(), "PHYSICAL".to_string());
        assert_eq!(PassthroughCardType::MerchantLocked.to_string(), "MERCHANT_LOCKED".to_string());
        assert_eq!(PassthroughCardType::SingleUse.to_string(), "SINGLE_USE".to_string());
    }

    #[test]
    fn test_conversion_passthrough_card_status() {
        assert_eq!(PassthroughCardStatus::Closed.to_string(), "CLOSED".to_string());
        assert_eq!(PassthroughCardStatus::Open.to_string(), "OPEN".to_string());
        assert_eq!(PassthroughCardStatus::Paused.to_string(), "PAUSED".to_string());
        assert_eq!(PassthroughCardStatus::PendingActivation.to_string(), "PENDING_ACTIVATION".to_string());
        assert_eq!(PassthroughCardStatus::PendingFulfillment.to_string(), "PENDING_FULFILLMENT".to_string());
    }

}
