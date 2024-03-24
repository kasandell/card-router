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
    // TODO: should this be a data error?
    pub fn convert(request: &AsaRequest) -> Result<Self, DataError> {
        let merchant = request.merchant.clone().ok_or(DataError::Format("missing merchant".into()))?;
        let descriptor = merchant.descriptor.clone().ok_or(DataError::Format("missing descriptor".into()))?;
        let mcc = merchant.mcc.clone().ok_or(DataError::Format("missing mcc".into()))?;
        let amount = request.amount.ok_or(DataError::Format("missing amount".into()))?;
        Ok(
            TransactionMetadata {
                memo: descriptor,
                amount_cents: amount,
                mcc: mcc
            }
        )
    }
}


#[cfg(test)]
mod test {
    use crate::asa::request::{AsaRequest, Merchant};
    use crate::common::model::TransactionMetadata;
    use crate::error::data_error::DataError;

    const AMOUNT: i32 = 100;
    const MCC: &str = "7184";

    const DESCRIPTOR: &str = "Kyle's Coffee";

    #[test]
    pub fn test_convert() {
        let req = AsaRequest {
            amount: Some(AMOUNT),
            acquirer_fee: None,
            authorization_amount: None,
            avs: None,
            card: None,
            cardholder_authentication: None,
            cash_amount: None,
            conversion_rate: None,
            created: None,
            events: None,
            funding: None,
            merchant_amount: None,
            merchant_currency: None,
            merchant: Some(Merchant {
                acceptor_id: None,
                city: None,
                country: None,
                descriptor: Some(DESCRIPTOR.to_string()),
                mcc: Some(MCC.to_string()),
                state: None,
            }),
            network: None,
            network_risk_score: None,
            pos: None,
            settled_amount: None,
            status: None,
            token: None,
            token_info: None,
        };

        let txn = TransactionMetadata::convert(&req).expect("Should be no error");
        assert_eq!(AMOUNT, txn.amount_cents);
        assert_eq!(DESCRIPTOR, txn.memo.as_str());
        assert_eq!(MCC, txn.mcc.as_str());
    }

    #[test]
    pub fn test_convert_throws_no_merchant() {
        let req = AsaRequest {
            amount: Some(AMOUNT),
            acquirer_fee: None,
            authorization_amount: None,
            avs: None,
            card: None,
            cardholder_authentication: None,
            cash_amount: None,
            conversion_rate: None,
            created: None,
            events: None,
            funding: None,
            merchant_amount: None,
            merchant_currency: None,
            merchant: None,
            network: None,
            network_risk_score: None,
            pos: None,
            settled_amount: None,
            status: None,
            token: None,
            token_info: None,
        };

        let error = TransactionMetadata::convert(&req).expect_err("Should be error");
        // Terrible conversion, but in test, errors are matched based on enum, not internals
        assert_eq!(DataError::Format("test".into()), error);
    }

    #[test]
    pub fn test_convert_throws_no_mcc() {
        let req = AsaRequest {
            amount: Some(AMOUNT),
            acquirer_fee: None,
            authorization_amount: None,
            avs: None,
            card: None,
            cardholder_authentication: None,
            cash_amount: None,
            conversion_rate: None,
            created: None,
            events: None,
            funding: None,
            merchant_amount: None,
            merchant_currency: None,
            merchant: Some(Merchant {
                acceptor_id: None,
                city: None,
                country: None,
                descriptor: Some(DESCRIPTOR.to_string()),
                mcc: None,
                state: None,
            }),
            network: None,
            network_risk_score: None,
            pos: None,
            settled_amount: None,
            status: None,
            token: None,
            token_info: None,
        };

        let error = TransactionMetadata::convert(&req).expect_err("Should be error");
        // Terrible conversion, but in test, errors are matched based on enum, not internals
        assert_eq!(DataError::Format("test".into()), error);

    }

    #[test]
    pub fn test_convert_throws_no_amount() {
        let req = AsaRequest {
            amount: None,
            acquirer_fee: None,
            authorization_amount: None,
            avs: None,
            card: None,
            cardholder_authentication: None,
            cash_amount: None,
            conversion_rate: None,
            created: None,
            events: None,
            funding: None,
            merchant_amount: None,
            merchant_currency: None,
            merchant: Some(Merchant {
                acceptor_id: None,
                city: None,
                country: None,
                descriptor: Some(DESCRIPTOR.to_string()),
                mcc: Some(MCC.to_string()),
                state: None,
            }),
            network: None,
            network_risk_score: None,
            pos: None,
            settled_amount: None,
            status: None,
            token: None,
            token_info: None,
        };

        let error = TransactionMetadata::convert(&req).expect_err("Should be error");
        // Terrible conversion, but in test, errors are matched based on enum, not internals
        assert_eq!(DataError::Format("test".into()), error);

    }

    #[test]
    pub fn test_convert_throws_no_descriptor() {
        let req = AsaRequest {
            amount: Some(AMOUNT),
            acquirer_fee: None,
            authorization_amount: None,
            avs: None,
            card: None,
            cardholder_authentication: None,
            cash_amount: None,
            conversion_rate: None,
            created: None,
            events: None,
            funding: None,
            merchant_amount: None,
            merchant_currency: None,
            merchant: Some(Merchant {
                acceptor_id: None,
                city: None,
                country: None,
                descriptor: None,
                mcc: None,
                state: None,
            }),
            network: None,
            network_risk_score: None,
            pos: None,
            settled_amount: None,
            status: None,
            token: None,
            token_info: None,
        };

        let error = TransactionMetadata::convert(&req).expect_err("Should be error");
        // Terrible conversion, but in test, errors are matched based on enum, not internals
        assert_eq!(DataError::Format("test".into()), error);
    }
}