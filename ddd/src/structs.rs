pub mod error_detail;
pub mod events;
pub mod ids;
pub mod invariant_error;
pub mod user_input_error;

#[cfg(feature = "axum")]
pub mod axum_extensions {

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
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
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StatusCodeErrors {
        pub errors: Vec<StatusCodeError>,
    }

    impl StatusCodeErrors {
        pub fn new(errors: Vec<StatusCodeError>) -> Self {
            Self { errors }
        }
    }
}
