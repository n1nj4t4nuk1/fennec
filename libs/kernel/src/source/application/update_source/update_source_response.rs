//! Response for the update-source use case.

use crate::source::application::find_source::find_source_response::SourceErrorEntry;

/// Response envelope returned by [`UpdateSourceCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`UpdateSourceCommandHandler`]: super::update_source_command_handler::UpdateSourceCommandHandler
pub struct UpdateSourceResponse {
    pub error: Option<SourceErrorEntry>,
}
