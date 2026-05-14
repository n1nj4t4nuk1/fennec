//! Domain service for finding a single source.

use std::sync::Arc;

use tracing::debug;

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::repositories::source_repository::SourceRepository;
use crate::source::domain::value_objects::source_id::SourceId;

/// Domain service that looks up a single [`Source`] by id.
///
/// Returns the domain entity directly. The handler is responsible for
/// mapping it to a response DTO.
pub struct SourceFinder {
    repository: Arc<dyn SourceRepository>,
}

impl SourceFinder {
    pub fn new(repository: Arc<dyn SourceRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: SourceId) -> Result<Source, SourceRepositoryError> {
        debug!(id = %id, "Finding source");
        let source = self.repository.find_by_id(&id).await?;

        source.ok_or(SourceRepositoryError::NotFound)
    }
}
