use super::error_detail::ErrorDetail;
use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantError<'a> {
    error_details: HashSet<ErrorDetail<'a>>,
}

impl<'a> InvariantError<'a> {
    pub fn new(error_details: HashSet<ErrorDetail<'a>>) -> Self {
        InvariantError { error_details }
    }

    pub fn error_details(&self) -> &HashSet<ErrorDetail<'a>> {
        &self.error_details
    }
}

impl<'a> Display for InvariantError<'a> {
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

impl<'a> std::error::Error for InvariantError<'a> {}

#[cfg(feature = "axum")]
pub mod axum_extensions {

    use axum::{Json, http::StatusCode, response::IntoResponse};

    use crate::structs::{
        invariant_error::InvariantError,
        status_code_error::axum_extensions::{StatusCodeError, StatusCodeErrors},
    };

    impl<'a> From<InvariantError<'a>> for StatusCodeErrors {
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

    impl<'a> IntoResponse for InvariantError<'a> {
        fn into_response(self) -> axum::response::Response {
            let error_response: StatusCodeErrors = self.into();
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
        }
    }
}
