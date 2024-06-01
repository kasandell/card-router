#[cfg(test)]
mod test {
    use crate::ledger::constant::ChargeStatus;

    #[test]
    pub fn test_charge_status_serializes() {
        assert_eq!("SUCCESS", ChargeStatus::Success.to_string());
        assert_eq!("FAIL", ChargeStatus::Fail.to_string());
    }
}