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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, AsExpression, FromSqlRow)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[diesel(sql_type = Text)]
pub enum WalletStatus {
    Active,
    Paused,
    Closed
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


impl ToSql<Text, Pg> for WalletStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for WalletStatus {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"CLOSED" => Ok(WalletStatus::Closed),
            b"PAUSED" => Ok(WalletStatus::Paused),
            b"ACTIVE" => Ok(WalletStatus::Active),
            v => Err(format!("Unknown value for WalletStatus found").into()),

        }
    }
}

impl fmt::Display for WalletStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            WalletStatus::Closed => "CLOSED",
            WalletStatus::Paused => "PAUSED",
            WalletStatus::Active => "ACTIVE",
        })
    }
}

impl WalletStatus {
    // TODO: is this the right spot for this?
    pub fn can_transition(&self, new_status: &Self) -> bool {
        match *new_status {
            WalletStatus::Active => *self == WalletStatus::Paused,
            WalletStatus::Paused => *self == WalletStatus::Active,
            WalletStatus::Closed => (
                *self == WalletStatus::Active
                || *self == WalletStatus::Paused
                )
        }

    }
}


#[cfg(test)]
mod test {
    use crate::wallet::constant::{WalletCardAttemptStatus, WalletStatus};

    #[test]
    pub fn test_wallet_card_attempt_status_serializes() {
        assert_eq!("FAILED", WalletCardAttemptStatus::Failed.to_string());
        assert_eq!("PENDING", WalletCardAttemptStatus::Pending.to_string());
        assert_eq!("MATCHED", WalletCardAttemptStatus::Matched.to_string());
    }

    #[test]
    pub fn test_wallet_status() {
        assert_eq!("ACTIVE", WalletStatus::Active.to_string());
        assert_eq!("PAUSED", WalletStatus::Paused.to_string());
        assert_eq!("CLOSED", WalletStatus::Closed.to_string());
    }

    #[test]
    pub fn test_transitions() {
        assert!(WalletStatus::Paused.can_transition(&WalletStatus::Active));
        assert!(WalletStatus::Paused.can_transition(&WalletStatus::Closed));
        assert!(!WalletStatus::Paused.can_transition(&WalletStatus::Paused));

        assert!(WalletStatus::Active.can_transition(&WalletStatus::Paused));
        assert!(WalletStatus::Active.can_transition(&WalletStatus::Closed));
        assert!(!WalletStatus::Active.can_transition(&WalletStatus::Active));

        assert!(!WalletStatus::Closed.can_transition(&WalletStatus::Closed));
        assert!(!WalletStatus::Closed.can_transition(&WalletStatus::Closed));
        assert!(!WalletStatus::Closed.can_transition(&WalletStatus::Closed));
    }
}