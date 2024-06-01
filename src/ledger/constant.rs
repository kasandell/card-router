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
use serde::{Serializer};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum MoneyMovementType {
    PassthroughCardReserve,
    PassthroughCardSettle,
    PassthroughCardRelease,
    WalletReserve,
    WalletSettle,
    WalletRelease
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum MoneyMovementDirection {
    Debit, // debits are for any money INTO an account
    Credit // credits are for any money OUT of an account
}

impl ToSql<Text, Pg> for MoneyMovementType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for MoneyMovementType {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"PASSTHROUGH_CARD_RESERVE" => Ok(MoneyMovementType::PassthroughCardReserve),
            b"PASSTHROUGH_CARD_RELEASE" => Ok(MoneyMovementType::PassthroughCardReserve),
            b"PASSTHROUGH_CARD_SETTLE" => Ok(MoneyMovementType::PassthroughCardReserve),
            b"WALLET_RESERVE" => Ok(MoneyMovementType::WalletReserve),
            b"WALLET_RELEASE" => Ok(MoneyMovementType::WalletRelease),
            b"WALLET_SETTLE" => Ok(MoneyMovementType::WalletSettle),
            v => Err(format!("Unknown value for MoneyMovementType found").into()),
        }
    }
}

impl fmt::Display for MoneyMovementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            MoneyMovementType::PassthroughCardReserve => "PASSTHROUGH_CARD_RESERVE",
            MoneyMovementType::PassthroughCardSettle => "PASSTHROUGH_CARD_SETTLE",
            MoneyMovementType::PassthroughCardRelease => "PASSTHROUGH_CARD_RELEASE",
            MoneyMovementType::WalletReserve => "WALLET_RESERVE",
            MoneyMovementType::WalletSettle => "WALLET_SETTLE",
            MoneyMovementType::WalletRelease => "WALLET_RELEASE",
        })
    }
}

impl ToSql<Text, Pg> for MoneyMovementDirection {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for MoneyMovementDirection {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"CREDIT" => Ok(MoneyMovementDirection::Credit),
            b"DEBIT" => Ok(MoneyMovementDirection::Debit),
            v => Err(format!("Unknown value for MoneyMovementDirection found").into()),
        }
    }
}

impl fmt::Display for MoneyMovementDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            MoneyMovementDirection::Credit => "CREDIT",
            MoneyMovementDirection::Debit => "DEBIT",
        })
    }
}