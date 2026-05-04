use std::collections::HashSet;

use crate::building_blocks::error::error_detail::ErrorDetail;

/// A DomainError is any error that is a violation in the business rules/invariant
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    Single {
        /// The description of the error
        error_detail: ErrorDetail,
    },
    Multiple {
        /// The descriptions of the errors
        error_details: HashSet<ErrorDetail>,
    },
}

impl DomainError {
    /// Creates a DomainError::Single
    /// # Arguments
    /// * `error_detail` - The detail describing the DomainError
    pub fn single(error_detail: ErrorDetail) -> Self {
        Self::Single { error_detail }
    }

    /// Creates a DomainError::Multiple
    /// # Arguments
    /// * `error_details` - The details describing the DomainError
    pub fn multiple(error_details: HashSet<ErrorDetail>) -> Self {
        Self::Multiple { error_details }
    }

    /// The ErrorDetail describing the DomainError
    pub fn error_details(&self) -> Box<dyn Iterator<Item = &ErrorDetail> + '_> {
        match self {
            Self::Single { error_detail } => Box::new(std::iter::once(error_detail)),
            Self::Multiple { error_details } => Box::new(error_details.iter()),
        }
    }
}

impl From<ErrorDetail> for DomainError {
    fn from(value: ErrorDetail) -> Self {
        Self::single(value)
    }
}

impl From<HashSet<ErrorDetail>> for DomainError {
    fn from(value: HashSet<ErrorDetail>) -> Self {
        if value.len() == 1 {
            value.into_iter().next().unwrap().into()
        } else {
            Self::multiple(value)
        }
    }
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.error_details().peekable();

        while let Some(detail) = iter.next() {
            // Write directly to the formatter buffer
            write!(f, "{}: {}", detail.key(), detail.message())?;

            // Add a separator if there are more errors coming
            if iter.peek().is_some() {
                write!(f, "; ")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for DomainError {}
