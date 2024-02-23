use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCardRequest {
    pub stripe_payment_method_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAttemptRequest {
    pub expected_reference_id: String,
    pub credit_card_type_public_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchAttemptRequest {
    pub merchant_reference_id: String, // our system's identifier
    pub original_psp_reference: String, // psp reference from initial adyen request
    pub psp_reference: String, // psp reference, which is the current card to add
}