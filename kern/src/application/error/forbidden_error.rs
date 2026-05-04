use crate::building_blocks::error::error_detail::ErrorDetail;

/// A ForbiddenError is an error that is returned when the executor of a business process is not
/// allowed to execute that process
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenError {
    /// The error detail that describes the ForbiddenError
    error_detail: ErrorDetail,
}

impl ForbiddenError {
    /// Creates a ForbiddenError
    /// # Arguments
    /// * `error_detail` - The error detail that describes the ForbiddenError
    pub fn new(error_detail: ErrorDetail) -> Self {
        Self { error_detail }
    }

    /// The error detail that describes the ForbiddenError
    pub fn error_detail(&self) -> &ErrorDetail {
        &self.error_detail
    }
}

impl std::fmt::Display for ForbiddenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = format!(
            "{}: {}",
            self.error_detail.key().to_owned(),
            self.error_detail.message().to_owned()
        );
        write!(f, "{}", error_string)
    }
}

impl std::error::Error for ForbiddenError {}
