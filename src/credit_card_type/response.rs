use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardTypeResponse {
    pub public_id: Uuid,
    pub card_name: String,
    pub issuer_name: String,
    pub card_type: String,
    pub card_image_url: String,
}