#[cfg(test)]
mod test {
    use crate::rule::constant::{DayOfMonth, RuleStatus};

    #[test]
    pub fn test_rule_status_serialize() {
       assert_eq!("ACTIVE", RuleStatus::Active.to_string());
        assert_eq!("INACTIVE", RuleStatus::Inactive.to_string());
    }

    #[test]
    pub fn test_day_of_month_serialize() {
        assert_eq!("FIRST", DayOfMonth::First.to_string());
        assert_eq!("LAST", DayOfMonth::Last.to_string());
    }
}