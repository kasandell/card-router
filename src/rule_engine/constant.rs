#[derive(Debug, PartialEq)]
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

    pub fn from_str(str: &str) -> Self {
        match str {
            "VALID" => RuleStatus::VALID,
            "INVALID" => RuleStatus::INVALID,
            _ => RuleStatus::INVALID
        }
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn from_str(str: &str) -> Self {
        match str {
            "FIRST" => DayOfMonth::FIRST,
            "LAST" => DayOfMonth::LAST,
            _ => DayOfMonth::LAST
        }
    }
}