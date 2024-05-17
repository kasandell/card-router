use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::wallet::constant::WalletStatus;
use crate::wallet::model::WalletWithExtraInfoModel;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WalletCardAttemptResponse {
    pub reference_id: String,
    pub token: String,
    pub expires_at: Option<String>
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WalletAddCardSuccessResponse {
    pub public_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateStatusResponse {
    pub public_id: Uuid,
    pub status: WalletStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayableCardInfo {
    pub public_id: Uuid,
    pub created_at: NaiveDateTime,
    pub card_name: String,
    pub issuer_name: String,
    pub card_type: String,
    pub card_image_url: String,
}

impl From<WalletWithExtraInfoModel> for DisplayableCardInfo {
    fn from(value: WalletWithExtraInfoModel) -> Self {
        DisplayableCardInfo {
            public_id: value.public_id,
            created_at: value.created_at,
            card_name: value.card_name,
            issuer_name: value.issuer_name,
            card_type: value.card_type,
            card_image_url: value.card_image_url
        }
    }
}
