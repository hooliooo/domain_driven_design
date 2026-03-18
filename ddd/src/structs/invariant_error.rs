use super::error_detail::ErrorDetail;
use std::{collections::HashSet, fmt::Display};

/// An InvarianError is a specific type of domain error that represents a violation of a core
/// business rule, which must always be true.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantError {
    /// The error details that describe that invariant error
    error_details: HashSet<ErrorDetail>,
}

impl InvariantError {
    /// Creates a new InvariantError
    /// # Arguments
    /// * `error_details`The error details that describe that invariant error
    pub fn new(error_details: HashSet<ErrorDetail>) -> Self {
        InvariantError { error_details }
    }

    /// The error details that describe that invariant error
    pub fn error_details(&self) -> &HashSet<ErrorDetail> {
        &self.error_details
    }
}

impl Display for InvariantError {
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

#[cfg(feature = "axum")]
pub mod axum_extensions {

    use axum::{Json, http::StatusCode, response::IntoResponse};

    use crate::structs::{
        invariant_error::InvariantError,
        status_code_error::axum_extensions::{StatusCodeError, StatusCodeErrors},
    };

    impl From<InvariantError> for StatusCodeErrors {
        fn from(value: InvariantError) -> Self {
            let errors: Vec<StatusCodeError> = value
                .error_details
                .into_iter()
                .map(|detail| {
                    StatusCodeError::new(detail.key().to_string(), detail.message().to_string())
                })
                .collect();
            StatusCodeErrors::new(errors)
        }
    }

    impl IntoResponse for InvariantError {
        fn into_response(self) -> axum::response::Response {
            let error_response: StatusCodeErrors = self.into();
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
        }
    }
}
