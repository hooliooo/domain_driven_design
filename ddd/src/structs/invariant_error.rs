use super::error_detail::ErrorDetail;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantError {
    error_details: HashSet<ErrorDetail>,
}

impl InvariantError {
    pub fn new(error_details: HashSet<ErrorDetail>) -> Self {
        InvariantError { error_details }
    }

    pub fn error_details(&self) -> &HashSet<ErrorDetail> {
        &self.error_details
    }
}

impl std::fmt::Display for InvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = self
            .error_details
            .iter()
            .fold(String::new(), |mut current, detail| {
                current.push_str(&format!(
                    "{}: {},",
                    detail.key().to_owned(),
                    detail.message().to_owned()
                ));
                current
            });

        write!(f, "{}", error_string)
    }
}

impl std::error::Error for InvariantError {}
