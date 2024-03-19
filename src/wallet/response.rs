use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
