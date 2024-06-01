use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::wallet::constant::{WalletCardAttemptStatus, WalletStatus};
use crate::wallet::entity::{Wallet, WalletCardAttempt, WalletWithExtraInfo};

#[derive(Clone, Debug, PartialEq)]
pub struct WalletModel {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub payment_method_id: String,
    pub created_at: NaiveDateTime,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
    pub status: WalletStatus
}

#[derive(Clone, Debug, PartialEq)]
pub struct WalletModelWithRule {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub payment_method_id: String,
    pub created_at: NaiveDateTime,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
    pub status: WalletStatus,
    pub rule_id: Option<i32>
}

#[derive(Clone, Debug, PartialEq)]
pub struct WalletCardAttemptModel {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub credit_card_id: i32,
    pub expected_reference_id: String,
    pub status: WalletCardAttemptStatus,
    pub created_at: NaiveDateTime,
}


#[derive(Clone, Debug, PartialEq)]
pub struct WalletWithExtraInfoModel {
    pub id: i32,
    pub public_id: Uuid,
    pub status: WalletStatus,
    pub created_at: NaiveDateTime,
    pub card_name: String,
    pub issuer_name: String,
    pub card_type: String,
    pub card_image_url: String,
}


impl From<Wallet> for WalletModel {
    fn from(value: Wallet) -> Self {
        WalletModel {
            id: value.id,
            public_id: value.public_id,
            user_id: value.user_id,
            payment_method_id: value.payment_method_id,
            created_at: value.created_at,
            credit_card_id: value.credit_card_id,
            wallet_card_attempt_id: value.wallet_card_attempt_id,
            status: value.status
        }
    }
}


impl From<WalletCardAttempt> for WalletCardAttemptModel {
    fn from(value: WalletCardAttempt) -> Self {
        WalletCardAttemptModel {
            id: value.id,
            public_id: value.public_id,
            user_id: value.user_id,
            credit_card_id: value.credit_card_id,
            expected_reference_id: value.expected_reference_id,
            status: value.status,
            created_at: value.created_at
        }
    }
}


impl From<WalletWithExtraInfo> for WalletWithExtraInfoModel {
    fn from(value: WalletWithExtraInfo) -> Self {
        WalletWithExtraInfoModel {
            id: value.id,
            public_id: value.public_id,
            status: value.status,
            created_at: value.created_at,
            card_name: value.card_name,
            issuer_name: value.issuer_name,
            card_type: value.card_type,
            card_image_url: value.card_image_url
        }
    }
}

impl From<WalletModel> for WalletModelWithRule {
    fn from(value: WalletModel) -> Self {
        WalletModelWithRule {
            id: value.id,
            public_id: value.public_id,
            user_id: value.user_id,
            payment_method_id: value.payment_method_id,
            created_at: value.created_at,
            credit_card_id: value.credit_card_id,
            wallet_card_attempt_id: value.wallet_card_attempt_id,
            status: value.status,
            rule_id: None,
        }
    }
}

impl From<WalletModelWithRule> for WalletModel {
    fn from(value: WalletModelWithRule) -> Self {
        WalletModel {
            id: value.id,
            public_id: value.public_id,
            user_id: value.user_id,
            payment_method_id: value.payment_method_id,
            created_at: value.created_at,
            credit_card_id: value.credit_card_id,
            wallet_card_attempt_id: value.wallet_card_attempt_id,
            status: value.status,
        }
    }
}