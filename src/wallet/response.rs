use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WalletCardAttemptResponse {
    pub public_id: Uuid,
}
