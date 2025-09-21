use std::{
    borrow::{Borrow, Cow},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct ErrorDetail<'a> {
    key: Cow<'a, str>,
    message: Cow<'a, str>,
}

impl<'a> ErrorDetail<'a> {
    pub fn new<K, M>(key: K, message: M) -> Self
    where
        K: Into<Cow<'a, str>>,
        M: Into<Cow<'a, str>>,
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

    pub const fn new_const(key: &'a str, message: &'a str) -> Self {
        ErrorDetail {
            key: Cow::Borrowed(key),
            message: Cow::Borrowed(message),
        }
    }
}

impl<'a> PartialEq for ErrorDetail<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<'a> Eq for ErrorDetail<'a> {}

impl<'a> Hash for ErrorDetail<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl<'a> Borrow<str> for ErrorDetail<'a> {
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
