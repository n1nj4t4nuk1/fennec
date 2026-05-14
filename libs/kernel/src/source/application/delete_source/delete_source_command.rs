//! Command for deleting a source.

use shared_cqrs::command::domain::command::Command;

use crate::source::domain::value_objects::source_id::SourceId;

/// Command that requests the deletion of a source by id.
pub struct DeleteSourceCommand {
    pub id: SourceId,
}

impl Command for DeleteSourceCommand {}
