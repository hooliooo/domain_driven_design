use crate::{
    application::error::forbidden_error::ForbiddenError,
    building_blocks::error::domain_error::DomainError,
};
use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct StatusCodeError {
    pub error_key: String,
    pub description: String,
}

impl StatusCodeError {
    pub fn new(error_key: String, description: String) -> Self {
        Self {
            error_key,
            description,
        }
    }

    pub fn error_key(&self) -> &String {
        &self.error_key
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct StatusCodeErrors {
    pub errors: Vec<StatusCodeError>,
}

impl StatusCodeErrors {
    pub fn new(errors: Vec<StatusCodeError>) -> Self {
        Self { errors }
    }
}

impl IntoResponse for DomainError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Single { error_detail } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(StatusCodeError::new(
                    error_detail.key().to_string(),
                    error_detail.message().to_string(),
                )),
            )
                .into_response(),
            Self::Multiple { error_details } => {
                let errors: Vec<StatusCodeError> = error_details
                    .into_iter()
                    .map(|detail| {
                        StatusCodeError::new(detail.key().to_string(), detail.message().to_string())
                    })
                    .collect();
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(StatusCodeErrors::new(errors)),
                )
                    .into_response()
            }
        }
    }
}

impl From<ForbiddenError> for StatusCodeError {
    fn from(value: ForbiddenError) -> Self {
        let detail = value.error_detail();
        StatusCodeError::new(detail.key().to_string(), detail.message().to_string())
    }
}

impl IntoResponse for ForbiddenError {
    fn into_response(self) -> axum::response::Response {
        let error_response: StatusCodeError = self.into();
        (StatusCode::FORBIDDEN, Json(error_response)).into_response()
    }
}
