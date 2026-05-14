//! Value Object for the last-update timestamp of a [`Source`].
//!
//! [`Source`]: crate::source::domain::entities::source::Source

use std::time::SystemTime;

/// An immutable Value Object wrapping a [`SystemTime`] that records when a
/// [`Source`](crate::source::domain::entities::source::Source) was last updated.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceUpdatedAt(SystemTime);

impl SourceUpdatedAt {
    /// Creates a new `SourceUpdatedAt` with the current system time.
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    /// Creates a `SourceUpdatedAt` from an explicit [`SystemTime`].
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
