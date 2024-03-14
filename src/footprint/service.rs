use std::env;
use std::sync::Arc;
use adyen_checkout::models::{Amount, PaymentCancelResponse, PaymentRequest, PaymentRequestPaymentMethod, PaymentResponse};
use async_trait::async_trait;
use crate::error::error_type::ErrorType;
use crate::error::service_error::ServiceError;
use footprint::apis::configuration::{ApiKey, BasicAuth, Configuration};
use footprint::apis::default_api::{post_vault_proxy, create_user_vault, create_client_token, post_vault_proxy_jit};
use footprint::models::{CreateClientTokenRequest, CreateClientTokenResponse, CreateUserVaultResponse};
use mockall::automock;
use serde_json::to_value;
use crate::constant::env_key::{ADYEN_API_KEY, FOOTPRINT_SECRET_KEY, FOOTPRINT_VAULT_PROXY_ID};
use crate::environment::ENVIRONMENT;
use crate::footprint::helper::{individual_request_part_for_customer_template, individual_request_part_for_customer_with_prefix_template, individual_request_part_for_customer_with_suffix_template};
use crate::footprint::r#enum::CardPart;
use crate::footprint::request::ChargeThroughProxyRequest;
use crate::constant::financial_constant;
use crate::user::entity::User;

#[automock]
#[async_trait(?Send)]
pub trait FootprintServiceTrait {
    async fn add_vault_for_user(self: Arc<Self>) -> Result<CreateUserVaultResponse, ServiceError>;
    async fn proxy_adyen_payment_request<'a>(self: Arc<Self>, request: &ChargeThroughProxyRequest<'a>) -> Result<PaymentResponse, ServiceError>;
    async fn create_client_token(self: Arc<Self>, user: &User, create_client_token_request: CreateClientTokenRequest) -> Result<CreateClientTokenResponse, ServiceError>;
    async fn proxy_adyen_cancel_request<'a>(self: Arc<Self>, psp_reference: &str) -> Result<PaymentCancelResponse, ServiceError>;
}

pub struct FootprintService {
    configuration: Configuration,
    adyen_proxy_id: String,
    adyen_api_key: String
}

impl FootprintService {
    #[tracing::instrument]
    pub fn new() -> Self {
        let mut cfg = Configuration::new();
        cfg.basic_auth = Some((env::var(FOOTPRINT_SECRET_KEY).expect("need key"), None));
        cfg.api_key = Some(ApiKey {
            prefix: None,
            key: env::var(FOOTPRINT_SECRET_KEY).expect("need key"),
        });
        Self {
            configuration: cfg,
            adyen_proxy_id: env::var(FOOTPRINT_VAULT_PROXY_ID).expect("Need a proxy id"),
            adyen_api_key: env::var(ADYEN_API_KEY).expect("Need an api key"),
        }
    }
}


#[async_trait(?Send)]
impl FootprintServiceTrait for FootprintService {
    #[tracing::instrument(skip(self))]
    async fn add_vault_for_user(self: Arc<Self>) ->  Result<CreateUserVaultResponse, ServiceError> {
        tracing::info!("Creating user vault");
        Ok(create_user_vault(
            &self.configuration
        ).await?)
    }

    #[tracing::instrument(skip(self))]
    async fn proxy_adyen_payment_request<'a>(self: Arc<Self>, request: &ChargeThroughProxyRequest<'a>) -> Result<PaymentResponse, ServiceError> {
        let number = Some(
            to_value(individual_request_part_for_customer_template(request.customer_public_id, request.payment_method_id, &CardPart::CardNumber))?
        );
        let cvc = Some(
            to_value(individual_request_part_for_customer_template(request.customer_public_id, request.payment_method_id, &CardPart::Cvc))?
        );
        let expiry_month = Some(
            to_value(individual_request_part_for_customer_with_prefix_template(request.customer_public_id, request.payment_method_id, &CardPart::Expiration))?
        );
        let expiry_year = Some(
            to_value(individual_request_part_for_customer_with_suffix_template(request.customer_public_id, request.payment_method_id, &CardPart::Expiration))?
        );
        let name = Some(
            to_value(individual_request_part_for_customer_template(request.customer_public_id, request.payment_method_id, &CardPart::Name))?
        );
        let payment_request = Some(PaymentRequest {
            account_info: None,
            additional_amount: None,
            additional_data: None,
            amount: Box::new(
                Amount {
                    currency: financial_constant::USD.to_string(),
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
            country_code: Some(financial_constant::US_COUNTRY_CODE.to_string()),
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
                    r#type: Some(Some(adyen_checkout::models::payment_request_payment_method::Type::Scheme)),//Some(Some(adyen_checkout::models::payment_request_payment_method::Type::Scheme)),
                    funding_source: None,
                    holder_name: Some(name),
                    brand: None,
                    cupsecureplus_period_smscode: None,
                    cvc: Some(cvc),
                    encrypted_card_number: None,
                    encrypted_expiry_month: None,
                    encrypted_expiry_year: None,
                    encrypted_security_code: None,
                    expiry_month: Some(expiry_month),
                    expiry_year: Some(expiry_year),
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
        /*
        let response = post_vault_proxy(
            &self.configuration,
            &self.adyen_proxy_id,
            &crate::footprint::constant::Constant::CONTENT_TYPE,
            &crate::footprint::constant::Constant::PROXY_METHOD,
            &crate::footprint::constant::Constant::PROXY_ACCESS_REASON,
            &self.adyen_api_key,
            Some(
                to_value(payment_request)?
            )
        ).await?; // post vault proxy response error needed

         */

        let response = post_vault_proxy_jit(
            &self.configuration,
            "application/json",
            "https://checkout-test.adyen.com/v71/payments",
            "POST",
            "charging",
            &self.adyen_api_key,
            Some(
                to_value(payment_request)?
            )
        ).await?;

        let payment_response: PaymentResponse = serde_json::from_value(response)?;
        Ok(payment_response)
    }

    #[tracing::instrument(skip(self))]
    async fn create_client_token(self: Arc<Self>, user: &User, create_client_token_request: CreateClientTokenRequest) -> Result<CreateClientTokenResponse, ServiceError> {
        Ok(
            create_client_token(
                &self.configuration,
                &user.footprint_vault_id,
                create_client_token_request
            ).await?
        )
    }

    #[tracing::instrument(skip(self))]
    async fn proxy_adyen_cancel_request<'a>(self: Arc<Self>, psp_reference: &str) -> Result<PaymentCancelResponse, ServiceError> {
        Err(
            ServiceError::new(
                ErrorType::InternalServerError, "Not implemented"
            )
        )
    }
}