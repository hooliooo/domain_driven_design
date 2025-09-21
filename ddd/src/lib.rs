pub mod enums;
pub mod macros;
pub mod structs;
pub mod traits;

// #[cfg(feature = "derive")]
pub use ddd_macros::*;
#[cfg(feature = "validator")]
pub mod validator_extensions {
    use std::collections::HashSet;

    use crate::{
        structs::{error_detail::ErrorDetail, invariant_error::InvariantError},
        traits::aggregate::Aggregate,
    };
    use validator::ValidationErrors;

    pub trait ResultValidation<T: Aggregate> {
        fn map_err_as_invariant_error<'a>(self) -> Result<T, InvariantError>;
    }

    impl<T> ResultValidation<T> for Result<T, ValidationErrors>
    where
        T: Aggregate,
    {
        fn map_err_as_invariant_error<'a>(self) -> Result<T, InvariantError> {
            self.map_err(|err| {
                let errors = err
                    .0
                    .into_iter()
                    .filter_map(|(key, value)| {
                        if let validator::ValidationErrorsKind::Field(errors) = value {
                            Some((key, errors))
                        } else {
                            None
                        }
                    })
                    .flat_map(|(key, errors)| {
                        let type_name = T::type_name();
                        let error_key = {
                            let mut error_key =
                                String::with_capacity(16 + type_name.len() + key.len());
                            error_key.push_str("error.");
                            error_key.push_str(type_name);
                            error_key.push_str(".invalid-");
                            error_key.extend(
                                key.as_ref().chars().map(|c| if c == '_' { '-' } else { c }),
                            );
                            error_key
                        };
                        errors.into_iter().filter_map(move |error| {
                            error.message.as_deref().map(|message| {
                                ErrorDetail::new(
                                    error_key.clone(),
                                    format!("'{}' {}", key, message),
                                )
                            })
                        })
                    })
                    .collect::<HashSet<ErrorDetail>>();
                InvariantError::new(errors)
            })
        }
    }
}
