use std::{borrow::Borrow, hash::Hash};

#[derive(Debug, Clone)]
pub struct ErrorDetail {
    key: String,
    message: String,
}

impl ErrorDetail {
    pub fn new(key: String, message: String) -> Self {
        ErrorDetail { key, message }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl PartialEq for ErrorDetail {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for ErrorDetail {}

impl Hash for ErrorDetail {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl Borrow<str> for ErrorDetail {
    fn borrow(&self) -> &str {
        self.key()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::ErrorDetail;

    #[test]
    fn test_hashset_get() {
        let detail = ErrorDetail::new("error.user.invalid-name".into(), "Bad name".into());
        let mut details: HashSet<ErrorDetail> = HashSet::new();
        details.insert(detail);
        assert!(details.contains("error.user.invalid-name"))
    }
}
