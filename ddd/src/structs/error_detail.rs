use std::{
    borrow::{Borrow, Cow},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct ErrorDetail {
    key: Cow<'static, str>,
    message: Cow<'static, str>,
}

impl ErrorDetail {
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

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn message(&self) -> &str {
        &self.message
    }

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
