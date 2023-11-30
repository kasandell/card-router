//TODO: placeholder until lithic gets back about generating openapi
pub struct AsaRequest {
    pub amount: i32,
    pub acquirer_fee: i32,
    pub authorization_amount: i32,
    pub avs: Avs,
    pub card: Card,
    pub cardholder_authentication: CardholderAuthentication,
    pub cash_amount: i32,
    pub conversion_rate: f32,
    pub created: String,
    pub events: Vec<Event>,
    pub funding: Vec<Funding>,
    pub merchant_amount: i32,
    pub merchant_currency: String,
    pub merchant: Merchant,
    pub network: String,
    pub network_risk_score: i32,
    pub pos: POS,
    pub settled_amount: i32,
    pub status: String,
    pub token: String,
    pub token_info: TokenInfo
}

pub struct Card {

}

pub struct CardholderAuthentication {

}

pub struct TokenInfo {

}

pub struct Merchant {
    pub acceptor_id: String,
    pub city: String,
    pub country: String,
    pub descriptor: String,
    pub mcc: String,
    pub state: String
}

pub struct Avs {
    pub address: String,
    pub zipcode: String,

}

pub struct Event {

}

pub struct Funding {
    pub amount: i32,
    pub token: String,
    pub type_: String
}

pub struct POS {
    pub terminal: Terminal,
    pub entry_mode: EntryMode
}

pub struct Terminal {
    pub attended: bool,
    pub operator: String,
    pub on_premise: bool,
    pub pin_capability: String,
    pub type_: String,
    pub partial_approval_capable: bool,
}

pub struct EntryMode {
    pub pan: String,
    pub pin_entered: bool,
    pub cardholder: String,
    pub card: String
}


#[cfg(test)]
pub fn create_example_asa(amount_cents: i32, mcc_code: String) -> AsaRequest {
    AsaRequest {
        amount: amount_cents,
        acquirer_fee: 0,
        authorization_amount: 0,
        avs: Avs { address: "test address".to_string(), zipcode: "10017".to_string() },
        card: Card {},
        cardholder_authentication: CardholderAuthentication {},
        cash_amount: 0,
        conversion_rate: 0.0,
        created: "2023-12-01".to_string(),
        events: Vec::new(),
        funding: Vec::new(),
        merchant_amount: 0,
        merchant_currency: "USD".to_string(),
        merchant: Merchant { acceptor_id: "1".to_string(), city: "New York".to_string(), country: "USA".to_string(), descriptor: "test merchant".to_string(), mcc: mcc_code, state: "NY".to_string() },
        network: "Visa".to_string(),
        network_risk_score: 0,
        pos: POS { 
            terminal: Terminal { attended: true, operator: "".to_string(), on_premise: true, pin_capability: "yes".to_string(), type_: "pos".to_string(), partial_approval_capable: true }, 
            entry_mode: EntryMode { pan: "".to_string(), pin_entered: true, cardholder: "".to_string(), card: "1234".to_string() }
        },
        settled_amount: 0,
        status: "new".to_string(),
        token: "test_token".to_string(),
        token_info: TokenInfo {}
    }

}