//! Factory function for [`SourceUpdatedEvent`].

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::events::source_updated_event::SourceUpdatedEvent;

/// Creates a [`SourceUpdatedEvent`] from the updated and previous sources.
pub fn create_source_updated_event(
    updated: &Source,
    previous: &Source,
) -> Result<SourceUpdatedEvent, SourceRepositoryError> {
    Ok(SourceUpdatedEvent::new(
        updated.id().clone(),
        updated.status().clone(),
        previous.status().clone(),
        updated.description().clone(),
        previous.description().clone(),
        updated.updated_at().clone(),
    ))
}
