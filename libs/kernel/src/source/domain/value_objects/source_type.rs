//! Value Object for the type of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use thiserror::Error;

/// Errors returned when constructing a [`SourceType`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SourceTypeError {
    /// The given string does not match any known source type.
    #[error("invalid source type: {0}")]
    Invalid(String),
}

/// An immutable Value Object representing the type of a
/// [`Source`](crate::source::domain::entities::source::Source).
///
/// Only the variants enumerated here are accepted; constructing one from a
/// raw string goes through [`SourceType::from_str`], which validates the input.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SourceType {
    /// HTTP/HTTPS URL feed.
    Url,
}

impl SourceType {
    /// Parses a raw string into a `SourceType`.
    ///
    /// # Errors
    ///
    /// Returns [`SourceTypeError::Invalid`] if the value is not a recognised
    /// source type.
    pub fn from_str(value: &str) -> Result<Self, SourceTypeError> {
        match value {
            "url" => Ok(Self::Url),
            other => Err(SourceTypeError::Invalid(other.to_string())),
        }
    }

    /// Returns the canonical string representation of this source type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Url => "url",
        }
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
