//! Query for finding a source by id.

use shared_cqrs::query::domain::query::Query;

use crate::source::domain::value_objects::source_id::SourceId;

/// Query that requests a single source by its [`SourceId`].
pub struct FindSourceQuery {
    pub id: SourceId,
}

impl Query for FindSourceQuery {}
