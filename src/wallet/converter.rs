use super::request::{
    PaymentMethod,
    Type
};
use adyen_checkout::models::{
    PaymentRequestPaymentMethod as AdyenPaymentMethod,
    payment_request_payment_method::Type as AdyenType
};

impl From<PaymentMethod> for AdyenPaymentMethod {
    fn from(value: PaymentMethod) -> Self {
        AdyenPaymentMethod {
            checkout_attempt_id: match value.checkout_attempt_id {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            recurring_detail_reference: None,
            stored_payment_method_id: None,
            r#type: Some(Some(AdyenType::Scheme)),/*match value.r#type {
                Some(r#type) => Some(AdyenType::from(r#type)),
                None => None
            },*/
            funding_source: None,
            holder_name: None,
            brand: match value.brand {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            cupsecureplus_period_smscode: None,
            cvc: None,
            encrypted_card_number: match value.encrypted_card_number {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            encrypted_expiry_month: match value.encrypted_expiry_month {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            encrypted_expiry_year: match value.encrypted_expiry_year {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            encrypted_security_code: match value.encrypted_security_code {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            expiry_month: None,
            expiry_year: None,
            network_payment_reference: None,
            number: None,
            shopper_notification_reference: None,
            three_ds2_sdk_version: match value.three_d_s2_sdk_version {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
        }
    }
}


impl From<Type> for AdyenType {
    fn from(value: Type) -> Self {
        match value {
            Type::Scheme => AdyenType::Scheme,
            // TODO: this is so bad
            _ => panic!("Unsupported type")
        }
    }
}