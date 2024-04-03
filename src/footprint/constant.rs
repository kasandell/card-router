pub mod Constant {
    pub const CONTENT_TYPE: &str = "application/json";
    pub const PROXY_METHOD: &str = "POST";
    pub const PROXY_ACCESS_REASON: &str = "Charge Proxy";
    pub const PROXY_URL: &str = "https://checkout-test.adyen.com/v71/payments";

    pub const TTL: i32 = 120; // 120 seconds to create a card after issuing token
}

#[cfg(test)]
mod test {
    use crate::footprint::constant::Constant;
    #[test]
    pub fn test_constants() {
        assert_eq!("POST", Constant::PROXY_METHOD);
        assert_eq!("application/json", Constant::CONTENT_TYPE);
        assert_eq!("Charge Proxy", Constant::PROXY_ACCESS_REASON);
        assert_eq!(120, Constant::TTL);
        assert_eq!("https://checkout-test.adyen.com/v71/payments", Constant::PROXY_URL);
    }
}