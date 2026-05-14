//! Value Object for the creation timestamp of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when a
/// [`Source`](crate::source::domain::entities::source::Source) was created.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceCreatedAt(SystemTime);

impl SourceCreatedAt {
    /// Creates a new `SourceCreatedAt` with the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Creates a `SourceCreatedAt` from an explicit [`SystemTime`].
    ///
    /// Useful when reconstituting from a persistent store.
    pub fn from_system_time(value: SystemTime) -> Self {
        Self(value)
    }

    /// Returns the underlying [`SystemTime`].
    pub fn value(&self) -> SystemTime {
        self.0
    }
}
