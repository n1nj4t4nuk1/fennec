use kernel::source::domain::value_objects::source_type::SourceType;

pub struct SourceTypeMother;

#[allow(dead_code)]
impl SourceTypeMother {
    pub fn url() -> SourceType {
        SourceType::Url
    }

    pub fn random() -> SourceType {
        SourceType::Url
    }
}
