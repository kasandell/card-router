use std::fmt;
use std::io::Write;
use diesel::{AsExpression, deserialize, FromSqlRow, serialize};
use diesel::backend::{Backend, RawValue};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum WalletCardAttemptStatus {
    Pending,
    Failed,
    Matched
}

impl ToSql<Text, Pg> for WalletCardAttemptStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for WalletCardAttemptStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"PENDING" => Ok(WalletCardAttemptStatus::Pending),
            b"MATCHED" => Ok(WalletCardAttemptStatus::Matched),
            b"FAILED" => Ok(WalletCardAttemptStatus::Failed),
            v => Err(format!("Unknown value for WalletCardAttemtpStatus found").into()),

        }
    }
}

impl fmt::Display for WalletCardAttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            WalletCardAttemptStatus::Pending => "PENDING",
            WalletCardAttemptStatus::Matched => "MATCHED",
            WalletCardAttemptStatus::Failed => "FAILED",
        })
    }
}