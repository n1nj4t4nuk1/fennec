//! Response types for the find-source use case.

use std::time::SystemTime;

/// Data entry DTO for a source.
pub struct SourceEntry {
    pub id: String,
    pub source_type: String,
    pub status: String,
    pub description: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Structured error DTO for source operations.
pub struct SourceErrorEntry {
    pub message: String,
    pub concept: String,
}

/// Response envelope returned by [`FindSourceQueryHandler`].
///
/// On success, `source` contains the data and `error` is `None`.
/// On failure, `source` is `None` and `error` contains the structured error.
///
/// [`FindSourceQueryHandler`]: super::find_source_query_handler::FindSourceQueryHandler
pub struct FindSourceResponse {
    pub source: Option<SourceEntry>,
    pub error: Option<SourceErrorEntry>,
}
