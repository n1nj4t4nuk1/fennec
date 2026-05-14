use kernel::source::domain::entities::source::Source;
use kernel::source::domain::value_objects::source_created_at::SourceCreatedAt;
use kernel::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

use crate::src::source::domain::value_objects::mothers::source_description_mother::SourceDescriptionMother;
use crate::src::source::domain::value_objects::mothers::source_id_mother::SourceIdMother;
use crate::src::source::domain::value_objects::mothers::source_status_mother::SourceStatusMother;
use crate::src::source::domain::value_objects::mothers::source_type_mother::SourceTypeMother;

pub struct SourceMother;

#[allow(dead_code)]
impl SourceMother {
    pub fn random() -> Source {
        let created_at = SourceCreatedAt::now();
        let updated_at = SourceUpdatedAt::from_system_time(created_at.value());
        Source::new(
            SourceIdMother::random(),
            SourceTypeMother::random(),
            SourceStatusMother::random(),
            SourceDescriptionMother::random(),
            created_at,
            updated_at,
        )
    }
}
