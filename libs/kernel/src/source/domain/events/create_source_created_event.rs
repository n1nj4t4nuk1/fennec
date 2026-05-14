//! Factory function for [`SourceCreatedEvent`].

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::events::source_created_event::SourceCreatedEvent;

/// Creates a [`SourceCreatedEvent`] from the given source.
pub fn create_source_created_event(
    source: &Source,
) -> Result<SourceCreatedEvent, SourceRepositoryError> {
    Ok(SourceCreatedEvent::new(
        source.id().clone(),
        source.source_type().clone(),
        source.status().clone(),
        source.description().clone(),
        source.created_at().clone(),
        source.updated_at().clone(),
    ))
}
