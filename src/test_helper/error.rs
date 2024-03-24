pub fn serde_error() -> serde_json::Error {
    serde_json::from_str::<serde_json::Value>("{ ").expect_err("should not parse")
}