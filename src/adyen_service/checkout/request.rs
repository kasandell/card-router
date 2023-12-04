use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCardRequest {
    pub number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvc: String,
    pub holder_name: String,
    pub public_id: String // customer public id
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeCardRequest {
    pub amount_cents: i32,
    pub mcc: String,
    pub payment_method_id: String,
    pub customer_public_id: Uuid,
    pub idempotency_key: String,
    pub reference: String,
    pub statement: String,
}