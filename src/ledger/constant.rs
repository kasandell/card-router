use crate::rule::constant::DayOfMonth;
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
pub enum ChargeStatus {
    Fail,
    Success
}

impl ToSql<Text, Pg> for ChargeStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for ChargeStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
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