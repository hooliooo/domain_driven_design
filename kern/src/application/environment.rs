use std::fmt::Display;

use crate::building_blocks::value_object::ValueObject;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<&str> for Environment {
    type Error = InvalidEnvironmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "development" => Ok(Environment::Development),
            "staging" => Ok(Environment::Staging),
            "production" => Ok(Environment::Production),
            _ => Err(InvalidEnvironmentError(value.into())),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = InvalidEnvironmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl ValueObject for Environment {}

#[derive(Debug)]
pub struct InvalidEnvironmentError(String);

impl Display for InvalidEnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!(
            "Invalid Environment: {}. Valid values are 'development', 'staging', or 'production'.",
            self.0
        );
        write!(f, "{message}")
    }
}

impl std::error::Error for InvalidEnvironmentError {}
