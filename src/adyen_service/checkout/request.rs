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



#[derive(Debug)]
pub struct ChargeCardRequest<'a> {
    pub amount_cents: i32,
    pub mcc: &'a str,
    pub payment_method_id: &'a str,
    pub customer_public_id: &'a Uuid,
    pub idempotency_key: &'a Uuid,
    pub reference: &'a str,
    pub statement: &'a str,
}