use std::{borrow::Borrow, hash::Hash};

#[derive(Debug, Clone)]
pub struct ErrorDetail<T> {
    key: T,
    message: T,
}

impl<T> ErrorDetail<T> {
    pub fn new(key: T, message: T) -> Self {
        ErrorDetail { key, message }
    }

    pub fn key(&self) -> &T {
        &self.key
    }

    pub fn message(&self) -> &T {
        &self.message
    }
}

impl<'a> ErrorDetail<&'a str> {
    pub const fn new_const(key: &'a str, message: &'a str) -> Self {
        ErrorDetail { key, message }
    }
}

impl<T> PartialEq for ErrorDetail<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<T> Eq for ErrorDetail<T> where T: Eq {}

impl<T> Hash for ErrorDetail<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl Borrow<str> for ErrorDetail<&str> {
    fn borrow(&self) -> &str {
        self.key()
    }
}

impl Borrow<str> for ErrorDetail<String> {
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
        let mut details: HashSet<ErrorDetail<&str>> = HashSet::new();
        details.insert(detail);
        assert!(details.contains("error.user.invalid-name"))
    }

    #[test]
    fn test_hashset_get_for_string_error_detail() {
        let detail = ErrorDetail::new(
            "error.user.invalid-name".to_string(),
            "Bad name".to_string(),
        );
        let mut details: HashSet<ErrorDetail<String>> = HashSet::new();
        details.insert(detail);
        assert!(details.contains("error.user.invalid-name"))
    }
}
