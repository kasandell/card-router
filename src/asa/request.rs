use serde::{Deserialize, Serialize};
//TODO: placeholder until lithic gets back about generating openapi

//TODO: make this all optional
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AsaRequest {
    pub amount: Option<i32>,
    pub acquirer_fee: Option<i32>,
    pub authorization_amount: Option<i32>,
    pub avs: Option<Avs>,
    pub card: Option<Card>,
    pub cardholder_authentication: Option<CardholderAuthentication>,
    pub cash_amount: Option<i32>,
    pub conversion_rate: Option<f32>,
    pub created: Option<String>,
    pub events: Option<Vec<Event>>,
    pub funding: Option<Vec<Funding>>,
    pub merchant_amount: Option<i32>,
    pub merchant_currency: Option<String>,
    pub merchant: Option<Merchant>,
    pub network: Option<String>,
    pub network_risk_score: Option<i32>,
    pub pos: Option<POS>,
    pub settled_amount: Option<i32>,
    pub status: Option<String>,
    pub token: Option<String>,
    pub token_info: Option<TokenInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Card {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardholderAuthentication {

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenInfo {

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Merchant {
    pub acceptor_id: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub descriptor: Option<String>,
    pub mcc: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Avs {
    pub address: Option<String>,
    pub zipcode: Option<String>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Funding {
    pub amount: Option<i32>,
    pub token: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct POS {
    pub terminal: Option<Terminal>,
    pub entry_mode: Option<EntryMode>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Terminal {
    pub attended: Option<bool>,
    pub operator: Option<String>,
    pub on_premise: Option<bool>,
    pub pin_capability: Option<String>,
    pub type_: Option<String>,
    pub partial_approval_capable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntryMode {
    pub pan: Option<String>,
    pub pin_entered: Option<bool>,
    pub cardholder: Option<String>,
    pub card: Option<String>,
}


#[cfg(test)]
pub fn create_example_asa(amount_cents: i32, mcc_code: String) -> AsaRequest {
    AsaRequest {
        amount: Some(amount_cents),
        acquirer_fee: Some(0),
        authorization_amount: Some(0),
        avs: Some(Avs { address: Some("test address".to_string()), zipcode: Some("10017".to_string()) }),
        card: Some(Card {
            token: Some("abc123".to_string())
        }),
        cardholder_authentication: Some(CardholderAuthentication {}),
        cash_amount: Some(0),
        conversion_rate: Some(0.0),
        created: Some("2023-12-01".to_string()),
        events: Some(Vec::new()),
        funding: Some(Vec::new()),
        merchant_amount: Some(0),
        merchant_currency: Some("USD".to_string()),
        merchant: Some(Merchant { acceptor_id: Some("1".to_string()), city: Some("New York".to_string()), country: Some("USA".to_string()), descriptor: Some("test merchant".to_string()), mcc: Some(mcc_code), state: Some("NY".to_string()) }),
        network: Some("Visa".to_string()),
        network_risk_score: Some(0),
        pos: Some(POS {
            terminal: Some(Terminal { attended: Some(true), operator: Some("".to_string()), on_premise: Some(true), pin_capability: Some("yes".to_string()), type_: Some("pos".to_string()), partial_approval_capable: Some(true)}),
            entry_mode: Some(EntryMode { pan: Some("".to_string()), pin_entered: Some(true), cardholder: Some("".to_string()), card: Some("1234".to_string()) })
        }),
        settled_amount: Some(0),
        status: Some("new".to_string()),
        token: Some("test_token".to_string()),
        token_info: Some(TokenInfo {})
    }

}