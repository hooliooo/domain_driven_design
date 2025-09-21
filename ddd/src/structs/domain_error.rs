use crate::structs::error_detail::ErrorDetail;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainError {
    error_detail: ErrorDetail,
}

impl DomainError {
    pub fn new(error_detail: ErrorDetail) -> Self {
        Self { error_detail }
    }

    pub fn error_detail(&self) -> &ErrorDetail {
        &self.error_detail
    }
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = format!(
            "{}: {}",
            self.error_detail.key().to_owned(),
            self.error_detail.message().to_owned()
        );
        write!(f, "{}", error_string)
    }
}

impl std::error::Error for DomainError {}

#[cfg(feature = "axum")]
pub mod axum_extensions {

    use axum::{Json, http::StatusCode, response::IntoResponse};

    use crate::structs::{
        domain_error::DomainError, status_code_error::axum_extensions::StatusCodeError,
    };

    impl From<DomainError> for StatusCodeError {
        fn from(value: DomainError) -> Self {
            let detail = value.error_detail();
            StatusCodeError::new(detail.key().to_string(), detail.message().to_string())
        }
    }

    impl IntoResponse for DomainError {
        fn into_response(self) -> axum::response::Response {
            let error_response: StatusCodeError = self.into();
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
        }
    }
}
