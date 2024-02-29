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
            bank_account_number: None,
            bank_account_type: None,
            bank_location_id: None,
            checkout_attempt_id: match value.checkout_attempt_id {
                Some(v) => Some(Some(serde_json::Value::from(v))),
                None => None
            },
            encrypted_bank_account_number: None,
            encrypted_bank_location_id: None,
            owner_name: None,
            recurring_detail_reference: None,
            stored_payment_method_id: None,
            r#type: Some(AdyenType::Scheme),/*match value.r#type {
                Some(r#type) => Some(AdyenType::from(r#type)),
                None => None
            },*/
            billing_address: None,
            delivery_address: None,
            personal_details: None,
            amazon_pay_token: None,
            checkout_session_id: None,
            apple_pay_token: None,
            funding_source: None,
            holder_name: None,
            issuer: None,
            blik_code: None,
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
            first_name: None,
            last_name: None,
            shopper_email: None,
            telephone_number: None,
            google_pay_token: None,
            subtype: None,
            masterpass_transaction_id: None,
            order_id: None,
            payee_preferred: None,
            payer_id: None,
            payer_selected: None,
            virtual_payment_address: None,
            samsung_pay_token: None,
            iban: None,
            billing_sequence_number: None,
            visa_checkout_call_id: None,
            app_id: None,
            openid: None,
            click_and_collect: None,
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