use serde::{Deserialize, Serialize};
use crate::asa::request::AsaRequest;
use crate::error::data_error::DataError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub memo: String,
    pub amount_cents: i32,
    pub mcc: String
}


impl TransactionMetadata {
    pub fn convert(request: &AsaRequest) -> Result<Self, DataError> {
        let merchant = request.merchant.clone().ok_or(DataError::Format("missing field".into()))?;
        let descriptor = merchant.descriptor.clone().ok_or(DataError::Format("missing field".into()))?;
        let mcc = merchant.mcc.clone().ok_or(DataError::Format("missing field".into()))?;
        let amount = request.amount.ok_or(DataError::Format("missing field".into()))?;
        Ok(
            TransactionMetadata {
                memo: descriptor,
                amount_cents: amount,
                mcc: mcc
            }
        )
    }
}
