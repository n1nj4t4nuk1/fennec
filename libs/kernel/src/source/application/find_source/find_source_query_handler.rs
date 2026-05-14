//! [`QueryHandler`] for the find-source use case.

use async_trait::async_trait;
use shared_cqrs::query::domain::query_bus_error::QueryBusError;
use shared_cqrs::query::domain::query_handler::QueryHandler;

use crate::source::application::find_source::find_source_response::{
    SourceEntry, SourceErrorEntry,
};
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;

use super::find_source_query::FindSourceQuery;
use super::find_source_response::FindSourceResponse;
use super::source_finder::SourceFinder;

/// [`QueryHandler`] that processes [`FindSourceQuery`]s by delegating
/// to [`SourceFinder`].
pub struct FindSourceQueryHandler {
    finder: SourceFinder,
}

impl FindSourceQueryHandler {
    pub fn new(finder: SourceFinder) -> Self {
        Self { finder }
    }
}

#[async_trait]
impl QueryHandler<FindSourceQuery> for FindSourceQueryHandler {
    type Response = FindSourceResponse;

    async fn handle(&self, query: FindSourceQuery) -> Result<Self::Response, QueryBusError> {
        match self.finder.execute(query.id).await {
            Ok(source) => Ok(FindSourceResponse {
                source: Some(SourceEntry {
                    id: source.id().to_string(),
                    source_type: source.source_type().to_string(),
                    status: source.status().to_string(),
                    description: source.description().value().to_string(),
                    created_at: source.created_at().value(),
                    updated_at: source.updated_at().value(),
                }),
                error: None,
            }),
            Err(e) => {
                let concept = match &e {
                    SourceRepositoryError::NotFound => "NotFound",
                    SourceRepositoryError::AlreadyExists => "AlreadyExists",
                    SourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(FindSourceResponse {
                    source: None,
                    error: Some(SourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
