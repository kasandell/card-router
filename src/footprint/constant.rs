pub mod Constant {
    pub const CONTENT_TYPE: &str = "application/json";
    pub const PROXY_METHOD: &str = "POST";
    pub const PROXY_ACCESS_REASON: &str = "Charge Proxy";
    pub const PROXY_URL: &str = "https://checkout-test.adyen.com/v71/payments";

    pub const TTL: i32 = 120; // 120 seconds to create a card after issuing token
}