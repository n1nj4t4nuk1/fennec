//! Value Object for the human-readable description of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

/// An immutable Value Object wrapping a `String` that holds the description
/// of a [`Source`](crate::source::domain::entities::source::Source).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceDescription(String);

impl SourceDescription {
    /// Creates a new `SourceDescription` from a raw string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns a reference to the underlying description string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SourceDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
