use kernel::source::domain::value_objects::source_id::SourceId;
use uuid::Uuid;

pub struct SourceIdMother;

#[allow(dead_code)]
impl SourceIdMother {
    pub fn create(value: Uuid) -> SourceId {
        SourceId::from_uuid(value)
    }

    pub fn random() -> SourceId {
        SourceId::from_uuid(Uuid::new_v4())
    }
}
