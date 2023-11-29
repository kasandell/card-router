#[derive(Debug, PartialEq)]
pub enum RuleStatus {
    ACTIVE,
    INACTIVE
}

impl RuleStatus {
    pub fn as_str(&self) -> String {
        match self {
            RuleStatus::ACTIVE => "ACTIVE".to_string(),
            RuleStatus::INACTIVE => "INACTIVE".to_string(),
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "ACTIVE" => RuleStatus::ACTIVE,
            "INACTIVE" => RuleStatus::INACTIVE,
            _ => RuleStatus::ACTIVE
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DayOfMonth {
    FIRST,
    LAST,
}

impl DayOfMonth {
    pub fn as_str(&self) -> String {
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