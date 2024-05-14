pub enum Environment {
    Test,
    Local,
    Dev,
    Stg,
    Int,
    Prod
}

impl Environment {
    pub fn as_string(&self) -> &'static str {
        match &self {
            Environment::Test => "test",
            Environment::Local => "local",
            Environment::Dev => "dev",
            Environment::Stg => "stg",
            Environment::Int => "int",
            Environment::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "test" => Ok(Environment::Test),
            "dev" => Ok(Environment::Dev),
            "stg" => Ok(Environment::Stg),
            "int" => Ok(Environment::Int),
            "prod" => Ok(Environment::Prod),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            ))
        }
    }
}