use std::fmt::Display;

use crate::traits::value_object::ValueObject;

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
            _ => Err(InvalidEnvironmentError),
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
pub struct InvalidEnvironmentError;

impl Display for InvalidEnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Environment Error")
    }
}

impl std::error::Error for InvalidEnvironmentError {}
