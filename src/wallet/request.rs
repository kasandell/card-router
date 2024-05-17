use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::wallet::constant::WalletStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAttemptRequest {
    pub credit_card_type_public_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchRequest {
    pub reference_id: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub wallet_card_public_id: Uuid,
    pub status: WalletStatus
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod {
    //#[serde(rename = "encryptedExpiryYear", deserialize_with = "Option::deserialize")]
    pub encrypted_expiry_year: Option<String>,
    //#[serde(rename = "encryptedExpiryMonth", deserialize_with = "Option::deserialize")]
    pub encrypted_expiry_month: Option<String>,
    //#[serde(rename = "encryptedSecurityCode", deserialize_with = "Option::deserialize")]
    pub encrypted_security_code: Option<String>,
    //#[serde(rename = "encryptedCardNumber", deserialize_with = "Option::deserialize")]
    pub encrypted_card_number: Option<String>,
    //#[serde(rename = "brand", deserialize_with = "Option::deserialize")]
    pub brand: Option<String>,
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<String>,
    //#[serde(rename = "checkoutAttemptId", deserialize_with = "Option::deserialize")]
    pub checkout_attempt_id: Option<String>,
    //#[serde(rename = "threeDS2SdkVersion", deserialize_with = "Option::deserialize")]
    pub three_d_s2_sdk_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "scheme")]
    Scheme,
}

/* Redefine these explicitly.
 * reason being, we will likely upgrade the crates, and don't want to inadvertently break our own api
 * though format coming in from frontend should 1:1 backend, this will save us if it doesn't
 * and yes i know its not very DRY
 */
/*
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// The bank account number (without separators).
    #[serde(rename = "bankAccountNumber", deserialize_with = "Option::deserialize")]
    pub bank_account_number: Option<serde_json::Value>,
    /// The bank account type (checking, savings...).
    #[serde(rename = "bankAccountType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<Option<BankAccountType>>,
    /// The bank routing number of the account.
    #[serde(rename = "bankLocationId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bank_location_id: Option<Option<serde_json::Value>>,
    /// The checkout attempt identifier.
    #[serde(rename = "checkoutAttemptId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub checkout_attempt_id: Option<Option<serde_json::Value>>,
    /// Encrypted bank account number. The bank account number (without separators).
    #[serde(rename = "encryptedBankAccountNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encrypted_bank_account_number: Option<Option<serde_json::Value>>,
    /// Encrypted location id. The bank routing number of the account. The field value is `nil` in most cases.
    #[serde(rename = "encryptedBankLocationId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encrypted_bank_location_id: Option<Option<serde_json::Value>>,
    /// The name of the bank account holder.
    #[serde(rename = "ownerName", deserialize_with = "Option::deserialize")]
    pub owner_name: Option<serde_json::Value>,
    /// This is the `recurringDetailReference` returned in the response when you created the token.
    #[serde(rename = "recurringDetailReference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recurring_detail_reference: Option<Option<serde_json::Value>>,
    /// This is the `recurringDetailReference` returned in the response when you created the token.
    #[serde(rename = "storedPaymentMethodId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stored_payment_method_id: Option<Option<serde_json::Value>>,
    /// **ach**
    #[serde(rename = "type", deserialize_with = "Option::deserialize")]
    pub r#type: Option<Type>,
    /// The address where to send the invoice.
    #[serde(rename = "billingAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Option<serde_json::Value>>,
    /// The address where the goods should be delivered.
    #[serde(rename = "deliveryAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<Option<serde_json::Value>>,
    /// Shopper name, date of birth, phone number, and email address.
    #[serde(rename = "personalDetails", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<Option<serde_json::Value>>,
    /// This is the `amazonPayToken` that you obtained from the [Get Checkout Session](https://amazon-pay-acquirer-guide.s3-eu-west-1.amazonaws.com/v1/amazon-pay-api-v2/checkout-session.html#get-checkout-session) response. This token is used for API only integration specifically.
    #[serde(rename = "amazonPayToken", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amazon_pay_token: Option<Option<serde_json::Value>>,
    /// The `checkoutSessionId` is used to identify the checkout session at the Amazon Pay side. This field is required only for drop-in and components integration, where it replaces the amazonPayToken.
    #[serde(rename = "checkoutSessionId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub checkout_session_id: Option<Option<serde_json::Value>>,
    /// The stringified and base64 encoded `paymentData` you retrieved from the Apple framework.
    #[serde(rename = "applePayToken", deserialize_with = "Option::deserialize")]
    pub apple_pay_token: Option<serde_json::Value>,
    /// The funding source that should be used when multiple sources are available. For Brazilian combo cards, by default the funding source is credit. To use debit, set this value to **debit**.
    #[serde(rename = "fundingSource", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub funding_source: Option<Option<adyen_checkout::models::payment_request_payment_method::FundingSource>>,
    /// The name of the card holder.
    #[serde(rename = "holderName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Option<serde_json::Value>>,
    /// The shopper's bank. Specify this with the issuer value that corresponds to this bank.
    #[serde(rename = "issuer", deserialize_with = "Option::deserialize")]
    pub issuer: Option<serde_json::Value>,
    /// BLIK code consisting of 6 digits.
    #[serde(rename = "blikCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blik_code: Option<Option<serde_json::Value>>,
    /// Secondary brand of the card. For example: **plastix**, **hmclub**.
    #[serde(rename = "brand", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub brand: Option<Option<serde_json::Value>>,
    #[serde(rename = "cupsecureplus.smscode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cupsecureplus_period_smscode: Option<Option<serde_json::Value>>,
    /// The card verification code. Only collect raw card data if you are [fully PCI compliant](https://docs.adyen.com/development-resources/pci-dss-compliance-guide).
    #[serde(rename = "cvc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cvc: Option<Option<serde_json::Value>>,
    /// The encrypted card number.
    #[serde(rename = "encryptedCardNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encrypted_card_number: Option<Option<serde_json::Value>>,
    /// The encrypted card expiry month.
    #[serde(rename = "encryptedExpiryMonth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encrypted_expiry_month: Option<Option<serde_json::Value>>,
    /// The encrypted card expiry year.
    #[serde(rename = "encryptedExpiryYear", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encrypted_expiry_year: Option<Option<serde_json::Value>>,
    /// The encrypted card verification code.
    #[serde(rename = "encryptedSecurityCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encrypted_security_code: Option<Option<serde_json::Value>>,
    /// The card expiry month. Only collect raw card data if you are [fully PCI compliant](https://docs.adyen.com/development-resources/pci-dss-compliance-guide).
    #[serde(rename = "expiryMonth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<Option<serde_json::Value>>,
    /// The card expiry year. Only collect raw card data if you are [fully PCI compliant](https://docs.adyen.com/development-resources/pci-dss-compliance-guide).
    #[serde(rename = "expiryYear", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<Option<serde_json::Value>>,
    /// The network token reference. This is the [`networkTxReference`](https://docs.adyen.com/api-explorer/#/CheckoutService/latest/post/payments__resParam_additionalData-ResponseAdditionalDataCommon-networkTxReference) from the response to the first payment.
    #[serde(rename = "networkPaymentReference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub network_payment_reference: Option<Option<serde_json::Value>>,
    /// The card number. Only collect raw card data if you are [fully PCI compliant](https://docs.adyen.com/development-resources/pci-dss-compliance-guide).
    #[serde(rename = "number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number: Option<Option<serde_json::Value>>,
    /// The `shopperNotificationReference` returned in the response when you requested to notify the shopper. Used for recurring payment only.
    #[serde(rename = "shopperNotificationReference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shopper_notification_reference: Option<Option<serde_json::Value>>,
    /// Required for mobile integrations. Version of the 3D Secure 2 mobile SDK.
    #[serde(rename = "threeDS2SdkVersion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub three_ds2_sdk_version: Option<Option<serde_json::Value>>,
    /// The shopper's first name.
    #[serde(rename = "firstName", deserialize_with = "Option::deserialize")]
    pub first_name: Option<serde_json::Value>,
    /// The shopper's last name.
    #[serde(rename = "lastName", deserialize_with = "Option::deserialize")]
    pub last_name: Option<serde_json::Value>,
    ///
    #[serde(rename = "shopperEmail", deserialize_with = "Option::deserialize")]
    pub shopper_email: Option<serde_json::Value>,
    ///
    #[serde(rename = "telephoneNumber", deserialize_with = "Option::deserialize")]
    pub telephone_number: Option<serde_json::Value>,
    /// The `token` that you obtained from the [Google Pay API](https://developers.google.com/pay/api/web/reference/response-objects#PaymentData) `PaymentData` response.
    #[serde(rename = "googlePayToken", deserialize_with = "Option::deserialize")]
    pub google_pay_token: Option<serde_json::Value>,
    /// The type of flow to initiate.
    #[serde(rename = "subtype", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<Option<Subtype>>,
    /// The Masterpass ledger ID.
    #[serde(rename = "masterpassTransactionId", deserialize_with = "Option::deserialize")]
    pub masterpass_transaction_id: Option<serde_json::Value>,
    /// The unique ID associated with the order.
    #[serde(rename = "orderID", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Option<serde_json::Value>>,
    /// IMMEDIATE_PAYMENT_REQUIRED or UNRESTRICTED
    #[serde(rename = "payeePreferred", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payee_preferred: Option<Option<serde_json::Value>>,
    /// The unique ID associated with the payer.
    #[serde(rename = "payerID", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<Option<serde_json::Value>>,
    /// PAYPAL or PAYPAL_CREDIT
    #[serde(rename = "payerSelected", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payer_selected: Option<Option<serde_json::Value>>,
    /// The virtual payment address for UPI.
    #[serde(rename = "virtualPaymentAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_payment_address: Option<Option<serde_json::Value>>,
    /// The payload you received from the Samsung Pay SDK response.
    #[serde(rename = "samsungPayToken", deserialize_with = "Option::deserialize")]
    pub samsung_pay_token: Option<serde_json::Value>,
    /// The International Bank Account Number (IBAN).
    #[serde(rename = "iban", deserialize_with = "Option::deserialize")]
    pub iban: Option<serde_json::Value>,
    /// The sequence number for the debit. For example, send **2** if this is the second debit for the subscription. The sequence number is included in the notification sent to the shopper.
    #[serde(rename = "billingSequenceNumber", deserialize_with = "Option::deserialize")]
    pub billing_sequence_number: Option<serde_json::Value>,
    /// The Visa Click to Pay Call ID value. When your shopper selects a payment and/or a shipping address from Visa Click to Pay, you will receive a Visa Click to Pay Call ID.
    #[serde(rename = "visaCheckoutCallId", deserialize_with = "Option::deserialize")]
    pub visa_checkout_call_id: Option<serde_json::Value>,
    #[serde(rename = "appId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "openid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub openid: Option<Option<serde_json::Value>>,
    /// Set this to **true** if the shopper would like to pick up and collect their order, instead of having the goods delivered to them.
    #[serde(rename = "clickAndCollect", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub click_and_collect: Option<Option<serde_json::Value>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankAccountType {
    #[serde(rename = "balance")]
    Balance,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "savings")]
    Savings,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "ach_plaid")]
    AchPlaid,
    #[serde(rename = "afterpay_default")]
    AfterpayDefault,
    #[serde(rename = "afterpaytouch")]
    Afterpaytouch,
    #[serde(rename = "afterpay_b2b")]
    AfterpayB2b,
    #[serde(rename = "clearpay")]
    Clearpay,
    #[serde(rename = "amazonpay")]
    Amazonpay,
    #[serde(rename = "androidpay")]
    Androidpay,
    #[serde(rename = "applepay")]
    Applepay,
    #[serde(rename = "directdebit_GB")]
    DirectdebitGb,
    #[serde(rename = "billdesk_online")]
    BilldeskOnline,
    #[serde(rename = "billdesk_wallet")]
    BilldeskWallet,
    #[serde(rename = "onlinebanking_IN")]
    OnlinebankingIn,
    #[serde(rename = "wallet_IN")]
    WalletIn,
    #[serde(rename = "blik")]
    Blik,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "networkToken")]
    NetworkToken,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "cellulant")]
    Cellulant,
    #[serde(rename = "doku_mandiri_va")]
    DokuMandiriVa,
    #[serde(rename = "doku_cimb_va")]
    DokuCimbVa,
    #[serde(rename = "doku_danamon_va")]
    DokuDanamonVa,
    #[serde(rename = "doku_bni_va")]
    DokuBniVa,
    #[serde(rename = "doku_permata_lite_atm")]
    DokuPermataLiteAtm,
    #[serde(rename = "doku_bri_va")]
    DokuBriVa,
    #[serde(rename = "doku_bca_va")]
    DokuBcaVa,
    #[serde(rename = "doku_alfamart")]
    DokuAlfamart,
    #[serde(rename = "doku_indomaret")]
    DokuIndomaret,
    #[serde(rename = "doku_wallet")]
    DokuWallet,
    #[serde(rename = "doku_ovo")]
    DokuOvo,
    #[serde(rename = "dotpay")]
    Dotpay,
    #[serde(rename = "dragonpay_ebanking")]
    DragonpayEbanking,
    #[serde(rename = "dragonpay_otc_banking")]
    DragonpayOtcBanking,
    #[serde(rename = "dragonpay_otc_non_banking")]
    DragonpayOtcNonBanking,
    #[serde(rename = "dragonpay_otc_philippines")]
    DragonpayOtcPhilippines,
    #[serde(rename = "econtext_seveneleven")]
    EcontextSeveneleven,
    #[serde(rename = "econtext_stores")]
    EcontextStores,
    #[serde(rename = "onlineBanking_PL")]
    OnlineBankingPl,
    #[serde(rename = "eps")]
    Eps,
    #[serde(rename = "onlineBanking_SK")]
    OnlineBankingSk,
    #[serde(rename = "onlineBanking_CZ")]
    OnlineBankingCz,
    #[serde(rename = "giropay")]
    Giropay,
    #[serde(rename = "googlepay")]
    Googlepay,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "klarna")]
    Klarna,
    #[serde(rename = "klarnapayments")]
    Klarnapayments,
    #[serde(rename = "klarnapayments_account")]
    KlarnapaymentsAccount,
    #[serde(rename = "klarnapayments_b2b")]
    KlarnapaymentsB2b,
    #[serde(rename = "klarna_paynow")]
    KlarnaPaynow,
    #[serde(rename = "klarna_account")]
    KlarnaAccount,
    #[serde(rename = "klarna_b2b")]
    KlarnaB2b,
    #[serde(rename = "masterpass")]
    Masterpass,
    #[serde(rename = "mbway")]
    Mbway,
    #[serde(rename = "mobilepay")]
    Mobilepay,
    #[serde(rename = "molpay_ebanking_fpx_MY")]
    MolpayEbankingFpxMy,
    #[serde(rename = "molpay_ebanking_TH")]
    MolpayEbankingTh,
    #[serde(rename = "openinvoice")]
    Openinvoice,
    #[serde(rename = "afterpay_directdebit")]
    AfterpayDirectdebit,
    #[serde(rename = "atome_pos")]
    AtomePos,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "payu_IN_upi")]
    PayuInUpi,
    #[serde(rename = "paywithgoogle")]
    Paywithgoogle,
    #[serde(rename = "alipay")]
    Alipay,
    #[serde(rename = "multibanco")]
    Multibanco,
    #[serde(rename = "bankTransfer_IBAN")]
    BankTransferIban,
    #[serde(rename = "paybright")]
    Paybright,
    #[serde(rename = "paynow")]
    Paynow,
    #[serde(rename = "affirm")]
    Affirm,
    #[serde(rename = "affirm_pos")]
    AffirmPos,
    #[serde(rename = "trustly")]
    Trustly,
    #[serde(rename = "trustlyvector")]
    Trustlyvector,
    #[serde(rename = "oney")]
    Oney,
    #[serde(rename = "facilypay")]
    Facilypay,
    #[serde(rename = "facilypay_3x")]
    Facilypay3x,
    #[serde(rename = "facilypay_4x")]
    Facilypay4x,
    #[serde(rename = "facilypay_6x")]
    Facilypay6x,
    #[serde(rename = "facilypay_10x")]
    Facilypay10x,
    #[serde(rename = "facilypay_12x")]
    Facilypay12x,
    #[serde(rename = "unionpay")]
    Unionpay,
    #[serde(rename = "kcp_banktransfer")]
    KcpBanktransfer,
    #[serde(rename = "kcp_payco")]
    KcpPayco,
    #[serde(rename = "kcp_creditcard")]
    KcpCreditcard,
    #[serde(rename = "wechatpaySDK")]
    WechatpaySdk,
    #[serde(rename = "wechatpayQR")]
    WechatpayQr,
    #[serde(rename = "wechatpayWeb")]
    WechatpayWeb,
    #[serde(rename = "molpay_boost")]
    MolpayBoost,
    #[serde(rename = "payu_IN_cashcard")]
    PayuInCashcard,
    #[serde(rename = "payu_IN_nb")]
    PayuInNb,
    #[serde(rename = "upi_qr")]
    UpiQr,
    #[serde(rename = "paytm")]
    Paytm,
    #[serde(rename = "molpay_ebanking_VN")]
    MolpayEbankingVn,
    #[serde(rename = "paybybank")]
    Paybybank,
    #[serde(rename = "ebanking_FI")]
    EbankingFi,
    #[serde(rename = "molpay_ebanking_MY")]
    MolpayEbankingMy,
    #[serde(rename = "molpay_ebanking_direct_MY")]
    MolpayEbankingDirectMy,
    #[serde(rename = "swish")]
    Swish,
    #[serde(rename = "pix")]
    Pix,
    #[serde(rename = "walley")]
    Walley,
    #[serde(rename = "walley_b2b")]
    WalleyB2b,
    #[serde(rename = "alma")]
    Alma,
    #[serde(rename = "paypo")]
    Paypo,
    #[serde(rename = "molpay_fpx")]
    MolpayFpx,
    #[serde(rename = "konbini")]
    Konbini,
    #[serde(rename = "directEbanking")]
    DirectEbanking,
    #[serde(rename = "boletobancario")]
    Boletobancario,
    #[serde(rename = "neteller")]
    Neteller,
    #[serde(rename = "paysafecard")]
    Paysafecard,
    #[serde(rename = "cashticket")]
    Cashticket,
    #[serde(rename = "ikano")]
    Ikano,
    #[serde(rename = "karenmillen")]
    Karenmillen,
    #[serde(rename = "oasis")]
    Oasis,
    #[serde(rename = "warehouse")]
    Warehouse,
    #[serde(rename = "primeiropay_boleto")]
    PrimeiropayBoleto,
    #[serde(rename = "mada")]
    Mada,
    #[serde(rename = "benefit")]
    Benefit,
    #[serde(rename = "knet")]
    Knet,
    #[serde(rename = "omannet")]
    Omannet,
    #[serde(rename = "gopay_wallet")]
    GopayWallet,
    #[serde(rename = "kcp_naverpay")]
    KcpNaverpay,
    #[serde(rename = "fawry")]
    Fawry,
    #[serde(rename = "atome")]
    Atome,
    #[serde(rename = "moneybookers")]
    Moneybookers,
    #[serde(rename = "naps")]
    Naps,
    #[serde(rename = "nordea")]
    Nordea,
    #[serde(rename = "boletobancario_bradesco")]
    BoletobancarioBradesco,
    #[serde(rename = "boletobancario_itau")]
    BoletobancarioItau,
    #[serde(rename = "boletobancario_santander")]
    BoletobancarioSantander,
    #[serde(rename = "boletobancario_bancodobrasil")]
    BoletobancarioBancodobrasil,
    #[serde(rename = "boletobancario_hsbc")]
    BoletobancarioHsbc,
    #[serde(rename = "molpay_maybank2u")]
    MolpayMaybank2u,
    #[serde(rename = "molpay_cimb")]
    MolpayCimb,
    #[serde(rename = "molpay_rhb")]
    MolpayRhb,
    #[serde(rename = "molpay_amb")]
    MolpayAmb,
    #[serde(rename = "molpay_hlb")]
    MolpayHlb,
    #[serde(rename = "molpay_affin_epg")]
    MolpayAffinEpg,
    #[serde(rename = "molpay_bankislam")]
    MolpayBankislam,
    #[serde(rename = "molpay_publicbank")]
    MolpayPublicbank,
    #[serde(rename = "fpx_agrobank")]
    FpxAgrobank,
    #[serde(rename = "touchngo")]
    Touchngo,
    #[serde(rename = "maybank2u_mae")]
    Maybank2uMae,
    #[serde(rename = "duitnow")]
    Duitnow,
    #[serde(rename = "promptpay")]
    Promptpay,
    #[serde(rename = "twint_pos")]
    TwintPos,
    #[serde(rename = "alipay_hk")]
    AlipayHk,
    #[serde(rename = "alipay_hk_web")]
    AlipayHkWeb,
    #[serde(rename = "alipay_hk_wap")]
    AlipayHkWap,
    #[serde(rename = "alipay_wap")]
    AlipayWap,
    #[serde(rename = "balanceplatform")]
    Balanceplatform,
    #[serde(rename = "ratepay")]
    Ratepay,
    #[serde(rename = "ratepay_directdebit")]
    RatepayDirectdebit,
    #[serde(rename = "samsungpay")]
    Samsungpay,
    #[serde(rename = "sepadirectdebit")]
    Sepadirectdebit,
    #[serde(rename = "sepadirectdebit_amazonpay")]
    SepadirectdebitAmazonpay,
    #[serde(rename = "bcmc_mobile")]
    BcmcMobile,
    #[serde(rename = "bcmc_mobile_QR")]
    BcmcMobileQr,
    #[serde(rename = "bcmc_mobile_app")]
    BcmcMobileApp,
    #[serde(rename = "momo_wallet")]
    MomoWallet,
    #[serde(rename = "momo_wallet_app")]
    MomoWalletApp,
    #[serde(rename = "twint")]
    Twint,
    #[serde(rename = "paymaya_wallet")]
    PaymayaWallet,
    #[serde(rename = "grabpay_SG")]
    GrabpaySg,
    #[serde(rename = "grabpay_MY")]
    GrabpayMy,
    #[serde(rename = "grabpay_TH")]
    GrabpayTh,
    #[serde(rename = "grabpay_ID")]
    GrabpayId,
    #[serde(rename = "grabpay_VN")]
    GrabpayVn,
    #[serde(rename = "grabpay_PH")]
    GrabpayPh,
    #[serde(rename = "oxxo")]
    Oxxo,
    #[serde(rename = "gcash")]
    Gcash,
    #[serde(rename = "dana")]
    Dana,
    #[serde(rename = "kakaopay")]
    Kakaopay,
    #[serde(rename = "truemoney")]
    Truemoney,
    #[serde(rename = "upi_collect")]
    UpiCollect,
    #[serde(rename = "upi_intent")]
    UpiIntent,
    #[serde(rename = "vipps")]
    Vipps,
    #[serde(rename = "visacheckout")]
    Visacheckout,
    #[serde(rename = "wechatpay")]
    Wechatpay,
    #[serde(rename = "wechatpay_pos")]
    WechatpayPos,
    #[serde(rename = "wechatpayMiniProgram")]
    WechatpayMiniProgram,
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "zip_pos")]
    ZipPos,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Subtype {
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "sdk")]
    Sdk,
}
 */