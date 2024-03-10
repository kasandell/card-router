use std::env;
use std::sync::Arc;
use adyen_checkout::models::{Amount, PaymentRequest, PaymentRequestPaymentMethod, PaymentResponse};
use async_trait::async_trait;
use crate::error_type::ErrorType;
use crate::footprint_service::response::AddVaultResponse;
use crate::service_error::ServiceError;
use footprint::apis::configuration::Configuration;
use footprint::apis::default_api::{
    post_vault_proxy,
    create_user_vault,
    create_client_token
};
use footprint::models::{
    CreateClientTokenResponse,
    CreateClientTokenRequest,
    CreateUserVaultResponse
};
use mockall::automock;
use serde_json::to_value;
use crate::constant::env_key::{ADYEN_API_KEY, FOOTPRINT_VAULT_PROXY_ID};
use crate::environment::ENVIRONMENT;
use crate::footprint_service::helper::individual_request_part_for_customer;
use crate::footprint_service::r#enum::CardPart;
use crate::footprint_service::request::ChargeThroughProxyRequest;

#[automock]
#[async_trait(?Send)]
pub trait FootprintServiceTrait {
    async fn add_vault_for_user(self: Arc<Self>) -> Result<CreateUserVaultResponse, ServiceError>;
    async fn proxy_adyen_payment_request<'a>(self: Arc<Self>, request: &ChargeThroughProxyRequest<'a>) -> Result<PaymentResponse, ServiceError>;
    async fn create_client_token(self: Arc<Self>, card_token: &str) -> Result<CreateClientTokenResponse, ServiceError>;
}

pub struct FootprintService {
    configuration: Configuration,
    adyen_proxy_id: String,
    adyen_api_key: String
}

impl FootprintService {
    pub fn new() -> Self {
        Self {
            configuration: Configuration::new(),
            adyen_proxy_id: env::var(FOOTPRINT_VAULT_PROXY_ID).expect("Need a proxy id"),
            adyen_api_key: env::var(ADYEN_API_KEY).expect("Need an api key"),
        }
    }
}


#[async_trait(?Send)]
impl FootprintServiceTrait for FootprintService {
    async fn add_vault_for_user(self: Arc<Self>) ->  Result<CreateUserVaultResponse, ServiceError> {
        Ok(create_user_vault(
            &self.configuration
        ).await?)
    }

    async fn proxy_adyen_payment_request<'a>(self: Arc<Self>, request: &ChargeThroughProxyRequest<'a>) -> Result<PaymentResponse, ServiceError> {
        let number = Some(
            to_value(individual_request_part_for_customer(request.customer_public_id, request.payment_method_id, &CardPart::CardNumber))?
        );
        let cvc = Some(
            to_value(individual_request_part_for_customer(request.customer_public_id, request.payment_method_id, &CardPart::Cvc))?
        );
        let expiry = Some(
            to_value(individual_request_part_for_customer(request.customer_public_id, request.payment_method_id, &CardPart::Expiration))?
        );
        let name = Some(
            to_value(individual_request_part_for_customer(request.customer_public_id, request.payment_method_id, &CardPart::Name))?
        );
        let payment_request = Some(PaymentRequest {
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
            mcc: Some(request.mcc.to_string()), // TODO: this causes txn to block
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
                    stored_payment_method_id: None,//Some(Some(to_value(request.payment_method_id)?)),
                    r#type: None,//Some(Some(adyen_checkout::models::payment_request_payment_method::Type::Scheme)),
                    funding_source: None,
                    holder_name: Some(name),
                    brand: None,
                    cupsecureplus_period_smscode: None,
                    cvc: Some(cvc),
                    encrypted_card_number: None,
                    encrypted_expiry_month: None,
                    encrypted_expiry_year: None,
                    encrypted_security_code: None,
                    expiry_month: None, // TODO: need to find a way to split this
                    expiry_year: Some(expiry),
                    network_payment_reference: None,
                    number: Some(number),
                    shopper_notification_reference: None,
                    three_ds2_sdk_version: None,
                }
            ),
            platform_chargeback_logic: None,
            recurring_expiry: None,
            recurring_frequency: None,
            recurring_processing_model: None,//Some(adyen_checkout::models::payment_request::RecurringProcessingModel::UnscheduledCardOnFile),
            redirect_from_issuer_method: None,
            redirect_to_issuer_method: None,
            reference: request.reference.to_string(),
            return_url: "".to_string(),
            risk_data: None,
            session_validity: None,
            shopper_email: None,
            shopper_ip: None,
            shopper_interaction: None,//Some(ShopperInteraction::ContAuth), // needed to run the payment
            shopper_locale: None,
            shopper_name: None,
            shopper_reference: Some(request.customer_public_id.to_string()),
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
        });
        let response = post_vault_proxy(
            &self.configuration,
            &self.adyen_proxy_id,
            &crate::footprint_service::constant::Constant::CONTENT_TYPE,
            &crate::footprint_service::constant::Constant::PROXY_METHOD,
            &crate::footprint_service::constant::Constant::PROXY_ACCESS_REASON,
            &self.adyen_api_key,
            Some(
                to_value(payment_request)?
            )
        ).await?; // post vault proxy response error needed

        let payment_response: PaymentResponse = serde_json::from_value(response)?;
        Ok(payment_response)
    }

    async fn create_client_token(self: Arc<Self>, card_token: &str) -> Result<CreateClientTokenResponse, ServiceError> {
        Ok(
            create_client_token(
                &self.configuration,
                card_token
            ).await?
        )
    }
}