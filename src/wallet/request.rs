use adyen_checkout::models::payment_method::FundingSource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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


#[derive(Debug, Serialize, Deserialize)]
pub struct AddCardRequest {
    pub credit_card_type_public_id: Uuid,
    pub payment_method: PaymentMethod
}

/* Redefine these explicitly.
 * reason being, we will likely upgrade the crates, and don't want to inadvertently break our own api
 * though format coming in from frontend should 1:1 backend, this will save us if it doesn't
 * and yes i know its not very DRY
 */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// Brand for the selected gift card. For example: plastix, hmclub.
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "brands", skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<String>>,
    /// The configuration of the payment method.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<::std::collections::HashMap<String, String>>>,
    /// The funding source of the payment method.
    #[serde(rename = "fundingSource", skip_serializing_if = "Option::is_none")]
    pub funding_source: Option<FundingSource>,
    /// The group where this payment method belongs to.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<PaymentMethodGroup>>,
    #[serde(rename = "inputDetails", skip_serializing_if = "Option::is_none")]
    pub input_details: Option<Vec<InputDetail>>,
    #[serde(rename = "issuers", skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<PaymentMethodIssuer>>,
    /// The displayable name of this payment method.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique payment method code.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodGroup {
    /// The name of the group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Echo data to be used if the payment method is displayed as part of this group.
    #[serde(rename = "paymentMethodData", skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<String>,
    /// The unique code of the group.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDetail {
    /// Configuration parameters for the required input.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<::std::collections::HashMap<String, String>>>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<SubInputDetail>>,
    #[serde(rename = "inputDetails", skip_serializing_if = "Option::is_none")]
    pub input_details: Option<Vec<SubInputDetail>>,
    /// In case of a select, the URL from which to query the items.
    #[serde(rename = "itemSearchUrl", skip_serializing_if = "Option::is_none")]
    pub item_search_url: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    /// The value to provide in the result.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// True if this input value is optional.
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// The type of the required input.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The value can be pre-filled, if available.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubInputDetail {
    /// Configuration parameters for the required input.
    #[serde(rename = "configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Option<::std::collections::HashMap<String, String>>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    /// The value to provide in the result.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// True if this input is optional to provide.
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// The type of the required input.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The value can be pre-filled, if available.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// The value to provide in the result.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The display name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethodIssuer {
    /// A boolean value indicating whether this issuer is unavailable. Can be `true` whenever the issuer is offline.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The unique identifier of this issuer, to submit in requests to /payments.
    #[serde(rename = "id")]
    pub id: String,
    /// A localized name of the issuer.
    #[serde(rename = "name")]
    pub name: String,
}