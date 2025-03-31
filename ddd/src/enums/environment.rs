use crate::traits::value_object::ValueObject;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Environment {
    Development,
    Staging,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Staging => "staging",
            Environment::Production => "production"
        }
    }
}

impl ValueObject for Environment {}
