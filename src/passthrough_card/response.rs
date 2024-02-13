use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HasActiveResponse {
    pub has_active: bool
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PassthroughCardResposnse {
    pub public_id: Uuid,
    pub card_status: String,
    pub card_type: String,
    pub last_four: String,
}
