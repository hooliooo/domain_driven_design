use super::error_detail::ErrorDetail;
use std::{collections::HashSet, fmt::Display};

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
