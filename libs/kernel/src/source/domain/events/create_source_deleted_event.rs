//! Factory function for [`SourceDeletedEvent`].

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::events::source_deleted_event::SourceDeletedEvent;

/// Creates a [`SourceDeletedEvent`] from the deleted source.
pub fn create_source_deleted_event(
    source: &Source,
) -> Result<SourceDeletedEvent, SourceRepositoryError> {
    Ok(SourceDeletedEvent::new(source.id().clone()))
}
