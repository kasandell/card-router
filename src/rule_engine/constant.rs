#[derive(Debug)]
pub enum RuleStatus {
    VALID,
    INVALID
}

impl RuleStatus {
    fn as_str(&self) -> String {
        match self {
            RuleStatus::VALID => "VALID".to_string(),
            RuleStatus::INVALID => "INVALID".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum DayOfMonth {
    FIRST,
    LAST,
}

impl DayOfMonth {
    fn as_str(&self) -> String {
        match self {
            DayOfMonth::FIRST => "FIRST".to_string(),
            DayOfMonth::LAST => "LAST".to_string(),
        }
    }
}