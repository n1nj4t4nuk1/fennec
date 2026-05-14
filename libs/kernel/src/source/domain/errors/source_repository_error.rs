//! Error types for source repository operations.

use thiserror::Error;

/// Errors that can occur during [`SourceRepository`] operations.
///
/// [`SourceRepository`]: crate::source::domain::repositories::source_repository::SourceRepository
#[derive(Debug, Error)]
pub enum SourceRepositoryError {
    /// The requested source was not found.
    #[error("source not found")]
    NotFound,

    /// A source with the same id already exists.
    #[error("source already exists")]
    AlreadyExists,

    /// An unexpected storage error occurred.
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
