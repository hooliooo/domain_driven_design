#[cfg(feature = "axum")]
pub mod axum_extensions {

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
}
