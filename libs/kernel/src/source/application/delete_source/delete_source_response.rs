//! Response for the delete-source use case.

use crate::source::application::find_source::find_source_response::SourceErrorEntry;

/// Response envelope returned by [`DeleteSourceCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`DeleteSourceCommandHandler`]: super::delete_source_command_handler::DeleteSourceCommandHandler
pub struct DeleteSourceResponse {
    pub error: Option<SourceErrorEntry>,
}
