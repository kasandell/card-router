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
use crate::ledger::constant::ChargeStatus;
use crate::rule::constant::RuleStatus::Inactive;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum RuleStatus {
    Active,
    Inactive
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum DayOfMonth {
    First,
    Last,
}


impl ToSql<Text, Pg> for RuleStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for RuleStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ACTIVE" => Ok(RuleStatus::Active),
            b"INACTIVE" => Ok(RuleStatus::Inactive),
            v => Err(format!("Unknown value for RuleStatus found").into()),

        }
    }
}

impl fmt::Display for RuleStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            RuleStatus::Active => "ACTIVE",
            RuleStatus::Inactive => "INACTIVE"
        })
    }
}


impl ToSql<Text, Pg> for DayOfMonth {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for DayOfMonth {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"FIRST" => Ok(DayOfMonth::First),
            b"LAST" => Ok(DayOfMonth::Last),
            v => Err(format!("Unknown value for DayOfMonth found").into()),

        }
    }
}

impl fmt::Display for DayOfMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            DayOfMonth::First => "FIRST",
            DayOfMonth::Last => "LAST"
        })
    }
}