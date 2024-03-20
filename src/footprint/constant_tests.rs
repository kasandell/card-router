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