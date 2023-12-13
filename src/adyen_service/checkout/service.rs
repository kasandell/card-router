use std::env;
use std::future::Future;

use adyen_checkout::{
    apis::{
        configuration::{
            ApiKey,
            Configuration
        },
        payments_api::post_payments
    }, models::{Amount, PaymentRequest, PaymentRequestPaymentMethod, PaymentResponse}
};
use adyen_checkout::apis::modifications_api::post_payments_payment_psp_reference_cancels;
use adyen_checkout::models::{PaymentCancelRequest, PaymentCancelResponse};
#[cfg(test)]
use mockall::{automock, predicate::*};
use serde_json::to_value;
use uuid::Uuid;

use crate::constant::env_key;

use super::error::Error;
use super::request::ChargeCardRequest;

pub struct ChargeService {}

#[cfg_attr(test, automock)]
pub trait AdyenChargeServiceTrait {
    fn test(&self) -> i32;
    //TODO: is making these not async going to make them run slower/block
    fn charge_card_on_file(
        &self,
        request: ChargeCardRequest
    ) -> Result<PaymentResponse, Error>;

    fn cancel_transaction(
        &self,
        psp_reference: &str,
    ) -> Result<PaymentCancelResponse, Error>;
}


//
impl AdyenChargeServiceTrait for ChargeService {
    fn test(&self) -> i32 {
        1
    }
    fn charge_card_on_file(
        &self,
        request: ChargeCardRequest
    ) -> Result<PaymentResponse, Error> {
        Ok(
            futures::executor::block_on( async {
                post_payments(
                    &Configuration {
                        api_key: Some(
                            ApiKey {
                                prefix: None,
                                key: env::var(env_key::ADYEN_API_KEY).expect("api key should exist")
                            }
                        ),
                        ..Default::default()
                    },
                    Some(to_value(request.idempotency_key)?),
                    Some(PaymentRequest {
                        account_info: None,
                        additional_amount: None,
                        additional_data: None,
                        amount: Box::new(
                            Amount {
                                currency: "USD".to_string(),
                                value: request.amount_cents as i64
                            }
                        ),
                        application_info: None,
                        authentication_data: None,
                        billing_address: None,
                        browser_info: None,
                        capture_delay_hours: None,
                        channel: None,
                        checkout_attempt_id: None,
                        company: None,
                        conversion_id: None,
                        country_code: None,
                        date_of_birth: None,
                        dcc_quote: None,
                        deliver_at: None,
                        delivery_address: None,
                        delivery_date: None,
                        device_fingerprint: None,
                        enable_one_click: None,
                        enable_pay_out: None,
                        enable_recurring: None,
                        entity_type: None,
                        fraud_offset: None,
                        fund_origin: None,
                        fund_recipient: None,
                        industry_usage: None,
                        installments: None,
                        line_items: None,
                        localized_shopper_statement: None,
                        mandate: None,
                        mcc: Some(request.mcc),
                        merchant_account: env::var(env_key::ADYEN_MERCHANT_ACCOUNT_NAME).expect("merchant account should exist"),
                        merchant_order_reference: None,
                        merchant_risk_indicator: None,
                        metadata: None,
                        mpi_data: None,
                        order: None,
                        order_reference: None,
                        origin: None,
                        payment_method: Box::new(
                            PaymentRequestPaymentMethod {
                                bank_account_number: None,
                                bank_account_type: None,
                                bank_location_id: None,
                                checkout_attempt_id: None,
                                encrypted_bank_account_number: None,
                                encrypted_bank_location_id: None,
                                owner_name: None,
                                recurring_detail_reference: None,
                                stored_payment_method_id: Some(Some(to_value(request.payment_method_id)?)),
                                r#type: Some(adyen_checkout::models::payment_request_payment_method::Type::Scheme),
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
                                brand: None,
                                cupsecureplus_period_smscode: None,
                                cvc: None,
                                encrypted_card_number: None,
                                encrypted_expiry_month: None,
                                encrypted_expiry_year: None,
                                encrypted_security_code: None,
                                expiry_month: None,
                                expiry_year: None,
                                network_payment_reference: None,
                                number: None,
                                shopper_notification_reference: None,
                                three_ds2_sdk_version: None,
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
                        ),
                        platform_chargeback_logic: None,
                        recurring_expiry: None,
                        recurring_frequency: None,
                        recurring_processing_model: Some(adyen_checkout::models::payment_request::RecurringProcessingModel::CardOnFile),
                        redirect_from_issuer_method: None,
                        redirect_to_issuer_method: None,
                        reference: request.reference,
                        return_url: "".to_string(),
                        risk_data: None,
                        session_validity: None,
                        shopper_email: None,
                        shopper_ip: None,
                        shopper_interaction: None,
                        shopper_locale: None,
                        shopper_name: None,
                        shopper_reference: None,
                        shopper_statement: Some(
                            request.statement
                        ),
                        social_security_number: None,
                        splits: None,
                        store: None,
                        store_payment_method: None,
                        telephone_number: None,
                        three_ds2_request_data: None,
                        three_ds_authentication_only: None,
                        trusted_shopper: None
                    }
                    )
                ).await
            }
            )?
        )
    }

    fn cancel_transaction(
        &self,
        psp_reference: &str,
    ) -> Result<PaymentCancelResponse, Error> {
        Ok(
            futures::executor::block_on(async {
                post_payments_payment_psp_reference_cancels(
                    &Configuration {
                        api_key: Some(
                            ApiKey {
                                prefix: None,
                                key: env::var(env_key::ADYEN_API_KEY).expect("api key should exist")
                            }
                        ),
                        ..Default::default()
                    },
                    psp_reference,
                    Some(to_value(Uuid::new_v4().to_string())?),
                    Some(
                        PaymentCancelRequest {
                            application_info: None,
                            merchant_account: env::var(env_key::ADYEN_MERCHANT_ACCOUNT_NAME).expect("merchant account should exist"),
                            reference: Some(Uuid::new_v4().to_string()),
                        }
                    )
                ).await
            })?
        )
    }
}

