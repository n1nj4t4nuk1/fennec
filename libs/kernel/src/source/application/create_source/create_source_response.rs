//! Response for the create-source use case.

use crate::source::application::find_source::find_source_response::SourceErrorEntry;

/// Response envelope returned by [`CreateSourceCommandHandler`].
///
/// On success, `error` is `None`. On failure, `error` contains the structured error.
///
/// [`CreateSourceCommandHandler`]: super::create_source_command_handler::CreateSourceCommandHandler
pub struct CreateSourceResponse {
    pub error: Option<SourceErrorEntry>,
}
