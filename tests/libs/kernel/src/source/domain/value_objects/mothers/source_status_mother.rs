use kernel::source::domain::value_objects::source_status::SourceStatus;

pub struct SourceStatusMother;

#[allow(dead_code)]
impl SourceStatusMother {
    pub fn active() -> SourceStatus {
        SourceStatus::Active
    }

    pub fn inactive() -> SourceStatus {
        SourceStatus::Inactive
    }

    pub fn random() -> SourceStatus {
        SourceStatus::Active
    }
}
