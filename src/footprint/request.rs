use uuid::Uuid;

#[derive(Debug)]
pub struct ChargeThroughProxyRequest<'a> {
    pub amount_cents: i32,
    pub mcc: &'a str,
    pub payment_method_id: &'a str,
    pub customer_public_id: &'a str,
    pub footprint_vault_id: &'a str,
    pub idempotency_key: &'a Uuid,
    pub reference: &'a str,
    pub statement: &'a str,
}