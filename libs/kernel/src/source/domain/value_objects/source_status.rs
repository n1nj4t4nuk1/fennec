//! Value Object for the status of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use thiserror::Error;

/// Errors returned when constructing a [`SourceStatus`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SourceStatusError {
    /// The given string does not match any known status.
    #[error("invalid source status: {0}")]
    Invalid(String),
}

/// An immutable Value Object representing the lifecycle status of a
/// [`Source`](crate::source::domain::entities::source::Source).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SourceStatus {
    /// The source is enabled and operating.
    Active,
    /// The source is disabled.
    Inactive,
}

impl SourceStatus {
    /// Parses a raw string into a `SourceStatus`.
    ///
    /// # Errors
    ///
    /// Returns [`SourceStatusError::Invalid`] if the value is not a recognised
    /// status.
    pub fn from_str(value: &str) -> Result<Self, SourceStatusError> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            other => Err(SourceStatusError::Invalid(other.to_string())),
        }
    }

    /// Returns the canonical string representation of this status.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}

impl std::fmt::Display for SourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
