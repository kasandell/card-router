use chrono::NaiveDateTime;

pub struct AddVaultResponse {
    pub id: String
}

pub struct ClientTokenResponse {
    pub expires_at: NaiveDateTime,
    pub token: String
}