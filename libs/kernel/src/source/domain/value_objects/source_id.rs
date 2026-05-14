//! Value Object for the unique identifier of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use uuid::Uuid;

/// An immutable Value Object wrapping a UUID v4 that uniquely identifies a
/// [`Source`](crate::source::domain::entities::source::Source).
///
/// The identifier is **externally provided** at construction time: callers
/// pass an already-generated UUID, this VO does not generate one.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SourceId(Uuid);

impl SourceId {
    /// Creates a new `SourceId` from a UUID supplied by the caller.
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying UUID.
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
