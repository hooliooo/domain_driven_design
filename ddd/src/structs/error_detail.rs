use std::{
    borrow::{Borrow, Cow},
    hash::Hash,
};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct ErrorDetail {
    key: &'static str,
    message: String,
}

impl ErrorDetail {
    pub fn new(key: &'static str, message: String) -> Self {
        ErrorDetail { key, message }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn by_key(self) -> ErrorDetailByKey {
        ErrorDetailByKey(self)
    }
}

#[derive(Debug)]
pub struct ErrorDetailByKey(ErrorDetail);
impl ErrorDetailByKey {
    pub fn message(&self) -> &String {
        self.0.message()
    }
}

impl Borrow<str> for ErrorDetailByKey {
    fn borrow(&self) -> &str {
        self.0.key()
    }
}

impl PartialEq for ErrorDetailByKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.key() == other.0.key()
    }
}

impl Eq for ErrorDetailByKey {}

impl Hash for ErrorDetailByKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.key().hash(state);
    }
}
