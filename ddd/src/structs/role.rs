use std::borrow::Borrow;

use crate::traits::value_object::ValueObject;

/// A Role is a value object that represents the authorization of a User
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Role {
    value: String,
}

impl Role {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

impl ValueObject for Role {}

impl Borrow<str> for Role {
    fn borrow(&self) -> &str {
        &self.value
    }
}

impl Borrow<String> for Role {
    fn borrow(&self) -> &String {
        &self.value
    }
}
