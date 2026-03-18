use std::{
    borrow::{Borrow, Cow},
    hash::Hash,
};

/// An ErrorDetail represents a key value pair that describes an error
#[derive(Debug, Clone)]
pub struct ErrorDetail {
    /// The key of the error
    key: Cow<'static, str>,
    /// The descriptive message of the error
    message: Cow<'static, str>,
}

impl ErrorDetail {
    /// Creates an ErrorDetail.
    /// # Arguments
    /// * `key` - The error key
    /// * `message` - The descriptive error message
    pub fn new<K, M>(key: K, message: M) -> Self
    where
        K: Into<Cow<'static, str>>,
        M: Into<Cow<'static, str>>,
    {
        ErrorDetail {
            key: key.into(),
            message: message.into(),
        }
    }

    /// The error key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// The descriptive error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Creates an ErrorDetail.
    /// # Arguments
    /// * `key` - The error key
    /// * `message` - The descriptive error message
    pub const fn new_const(key: &'static str, message: &'static str) -> Self {
        ErrorDetail {
            key: Cow::Borrowed(key),
            message: Cow::Borrowed(message),
        }
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
    fn test_hashset_get_for_str_error_detail() {
        let detail = ErrorDetail::new("error.user.invalid-name", "Bad name");
        let mut details: HashSet<ErrorDetail> = HashSet::new();
        details.insert(detail);
        assert!(details.contains("error.user.invalid-name"))
    }

    #[test]
    fn test_hashset_get_for_string_error_detail() {
        let detail = ErrorDetail::new(
            "error.user.invalid-name".to_string(),
            "Bad name".to_string(),
        );
        let mut details: HashSet<ErrorDetail> = HashSet::new();
        details.insert(detail);
        assert!(details.contains("error.user.invalid-name"))
    }
}
