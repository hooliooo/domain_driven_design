use crate::structs::error_detail::ErrorDetail;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserInputError<'a> {
    error_detail: ErrorDetail<'a>,
}

impl<'a> UserInputError<'a> {
    pub fn new(error_detail: ErrorDetail<'a>) -> Self {
        Self { error_detail }
    }

    pub fn error_detail(&self) -> &ErrorDetail<'a> {
        &self.error_detail
    }
}

impl<'a> std::fmt::Display for UserInputError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_string = format!(
            "{}: {}",
            self.error_detail.key().to_owned(),
            self.error_detail.message().to_owned()
        );
        write!(f, "{}", error_string)
    }
}

impl<'a> std::error::Error for UserInputError<'a> {}

#[cfg(feature = "axum")]
pub mod axum_extensions {

    use axum::{Json, http::StatusCode, response::IntoResponse};

    use crate::structs::{
        status_code_error::axum_extensions::StatusCodeError, user_input_error::UserInputError,
    };

    impl<'a> From<UserInputError<'a>> for StatusCodeError {
        fn from(value: UserInputError) -> Self {
            let detail = value.error_detail();
            StatusCodeError::new(detail.key().to_string(), detail.message().to_string())
        }
    }

    impl<'a> IntoResponse for UserInputError<'a> {
        fn into_response(self) -> axum::response::Response {
            let error_response: StatusCodeError = self.into();
            (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
        }
    }
}
