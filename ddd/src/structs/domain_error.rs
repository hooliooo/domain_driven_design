use crate::structs::error_detail::ErrorDetail;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainError<'a> {
    error_detail: ErrorDetail<'a>,
}

impl<'a> DomainError<'a> {
    pub fn new(error_detail: ErrorDetail<'a>) -> Self {
        Self { error_detail }
    }

    pub fn error_detail(&self) -> &ErrorDetail<'a> {
        &self.error_detail
    }
}

impl<'a> std::fmt::Display for DomainError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = format!(
            "{}: {}",
            self.error_detail.key().to_owned(),
            self.error_detail.message().to_owned()
        );
        write!(f, "{}", error_string)
    }
}

impl<'a> std::error::Error for DomainError<'a> {}

#[cfg(feature = "axum")]
pub mod axum_extensions {

    use axum::{Json, http::StatusCode, response::IntoResponse};

    use crate::structs::{
        domain_error::DomainError, status_code_error::axum_extensions::StatusCodeError,
    };

    impl<'a> From<DomainError<'a>> for StatusCodeError {
        fn from(value: DomainError) -> Self {
            let detail = value.error_detail();
            StatusCodeError::new(detail.key().to_string(), detail.message().to_string())
        }
    }

    impl<'a> IntoResponse for DomainError<'a> {
        fn into_response(self) -> axum::response::Response {
            let error_response: StatusCodeError = self.into();
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
        }
    }
}
