use crate::structs::error_detail::ErrorDetail;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserInputError {
    error_detail: ErrorDetail,
}

impl UserInputError {
    pub fn new(error_detail: ErrorDetail) -> Self {
        Self { error_detail }
    }

    pub fn error_detail(&self) -> &ErrorDetail {
        &self.error_detail
    }
}

impl std::fmt::Display for UserInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = format!(
            "{}: {}",
            self.error_detail.key().to_owned(),
            self.error_detail.message().to_owned()
        );
        write!(f, "{}", error_string)
    }
}

impl std::error::Error for UserInputError {}
