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