use std::collections::HashMap;
use std::env;

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
use adyen_checkout::models::payment_request::{RecurringProcessingModel, ShopperInteraction};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use serde_json::to_value;
use uuid::Uuid;

use crate::constant::env_key;
use crate::environment::ENVIRONMENT;

use crate::service_error::ServiceError;
use crate::user::entity::User;
use super::request::ChargeCardRequest;

pub struct ChargeService {
    configuration: Configuration
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AdyenChargeServiceTrait {
    //TODO: is making these not async going to make them run slower/block
    async fn charge_card_on_file<'a>(
        &self,
        request: &ChargeCardRequest<'a>
    ) -> Result<PaymentResponse, ServiceError>;

    async fn cancel_transaction(
        &self,
        psp_reference: &str,
    ) -> Result<PaymentCancelResponse, ServiceError>;

    // this is going to take a request in from the frontend, preformatted by adyen libraries
    // and pass it along
    async fn add_card(
        &self,
        idempotency_key: &str,
        user: &User,
        reference_id: &str,
        request: &PaymentRequestPaymentMethod
    ) -> Result<PaymentResponse, ServiceError>;
}

impl ChargeService {
    pub fn new() -> Self {
        Self {
            configuration: Configuration {
                api_key: Some(
                    ApiKey {
                        prefix: None,
                        key: ENVIRONMENT.adyen_api_key.clone()
                    }
                ),
                ..Default::default()
            }
        }
    }
}


#[async_trait]
impl AdyenChargeServiceTrait for ChargeService {
    async fn charge_card_on_file<'a>(
        &self,
        request: &ChargeCardRequest<'a>
    ) -> Result<PaymentResponse, ServiceError> {
        Ok(
            post_payments(
                &self.configuration,
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
                    mcc: Some(request.mcc.to_string()),
                    merchant_account: ENVIRONMENT.adyen_merchant_account_name.clone(),
                    merchant_order_reference: None,
                    merchant_risk_indicator: None,
                    metadata: None,
                    mpi_data: None,
                    order: None,
                    order_reference: None,
                    origin: None,
                    payment_method: Box::new(
                        PaymentRequestPaymentMethod {
                            checkout_attempt_id: None,
                            recurring_detail_reference: None,
                            stored_payment_method_id: Some(Some(to_value(request.payment_method_id)?)),
                            r#type: Some(Some(adyen_checkout::models::payment_request_payment_method::Type::Scheme)),
                            funding_source: None,
                            holder_name: None,
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
                        }
                    ),
                    platform_chargeback_logic: None,
                    recurring_expiry: None,
                    recurring_frequency: None,
                    recurring_processing_model: Some(adyen_checkout::models::payment_request::RecurringProcessingModel::CardOnFile),
                    redirect_from_issuer_method: None,
                    redirect_to_issuer_method: None,
                    reference: request.reference.to_string(),
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
                        request.statement.to_string()
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
            ).await?
        )
    }

    async fn cancel_transaction(
        &self,
        psp_reference: &str,
    ) -> Result<PaymentCancelResponse, ServiceError> {
        Ok(
            post_payments_payment_psp_reference_cancels(
                &self.configuration,
                psp_reference,
                Some(to_value(Uuid::new_v4().to_string())?),
                Some(
                    PaymentCancelRequest {
                        application_info: None,
                        merchant_account: ENVIRONMENT.adyen_merchant_account_name.clone(),
                        reference: Some(Uuid::new_v4().to_string()),
                    }
                )
            ).await?
        )
    }

    async fn add_card(
        &self,
        idempotency_key: &str,
        user: &User,
        reference_id: &str,
        request: &PaymentRequestPaymentMethod,
    ) -> Result<PaymentResponse, ServiceError> {
        println!("IN ADD CARD REQUEST");
        let additional_data: HashMap<String, String> =
            [("allow3DS2".to_string(), "true".to_string())].iter().cloned().collect();

        let obj = PaymentRequest {
            account_info: None,
            additional_amount: None,
            additional_data: None,
            amount: Box::new(Amount {
                currency: "USD".to_string(),
                value: 0
            }),
            application_info: None,
            authentication_data: None,
            billing_address: None,
            browser_info: None,
            capture_delay_hours: None,
            channel: None,
            checkout_attempt_id: None,
            company: None,
            conversion_id: None,
            country_code: Some("US".to_string()), // TODO: from request?
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
            mcc: None,
            merchant_account: ENVIRONMENT.adyen_merchant_account_name.clone(),
            merchant_order_reference: None,
            merchant_risk_indicator: None,
            metadata: None,
            mpi_data: None,
            order: None,
            order_reference: None,
            origin: None,
            // TODO: is this okay?
            payment_method: Box::new(request.clone()),
            platform_chargeback_logic: None,
            recurring_expiry: None,
            recurring_frequency: None,
            recurring_processing_model: Some(RecurringProcessingModel::CardOnFile),
            redirect_from_issuer_method: None,
            redirect_to_issuer_method: None,
            // this needs to be same as wallet match reference
            reference: reference_id.to_string(),
            return_url: "myapp://payment".to_string(),
            risk_data: None,
            session_validity: None,
            shopper_email: None,
            shopper_ip: None,
            shopper_interaction: Some(ShopperInteraction::Ecommerce),
            shopper_locale: None,
            shopper_name: None,
            // this should be customer public id
            shopper_reference: Some(user.public_id.to_string()),
            shopper_statement: None,
            social_security_number: None,
            splits: None,
            store: None,
            store_payment_method: Some(true),
            telephone_number: None,
            three_ds2_request_data: None,
            three_ds_authentication_only: None,
            trusted_shopper: None,
        };
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        Ok(
            post_payments(
                &self.configuration,
                None,//Some(to_value(idempotency_key.to_string())?),
                // TODO: make sure we transform this appropriately to only take in required components from frontend
                Some(obj)
            ).await?
        )
    }
}

