//! Domain event raised when a new source is created.

use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};

use crate::source::domain::value_objects::source_created_at::SourceCreatedAt;
use crate::source::domain::value_objects::source_description::SourceDescription;
use crate::source::domain::value_objects::source_id::SourceId;
use crate::source::domain::value_objects::source_status::SourceStatus;
use crate::source::domain::value_objects::source_type::SourceType;
use crate::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

/// Domain event raised when a new [`Source`] is successfully persisted.
///
/// [`Source`]: crate::source::domain::entities::source::Source
pub struct SourceCreatedEvent {
    base: DomainEventBase,
    pub id: SourceId,
    pub source_type: SourceType,
    pub status: SourceStatus,
    pub description: SourceDescription,
    pub created_at: SourceCreatedAt,
    pub updated_at: SourceUpdatedAt,
}

impl SourceCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.kernel.source.created";

    /// Creates a new `SourceCreatedEvent` with auto-generated metadata.
    pub fn new(
        id: SourceId,
        source_type: SourceType,
        status: SourceStatus,
        description: SourceDescription,
        created_at: SourceCreatedAt,
        updated_at: SourceUpdatedAt,
    ) -> Self {
        Self {
            base: DomainEventBase::new(id.to_string()),
            id,
            source_type,
            status,
            description,
            created_at,
            updated_at,
        }
    }
}

impl DomainEvent for SourceCreatedEvent {
    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }

    fn aggregate_id(&self) -> &str {
        &self.base.aggregate_id
    }

    fn event_id(&self) -> &str {
        &self.base.event_id
    }

    fn occurred_on(&self) -> SystemTime {
        self.base.occurred_on
    }
}
