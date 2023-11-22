use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCardRequest {
    pub stripe_payment_method_id: String
}