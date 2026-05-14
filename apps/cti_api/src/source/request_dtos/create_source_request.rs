use serde::Deserialize;

/// JSON request body for `POST /sources`.
#[derive(Deserialize)]
pub struct CreateSourceRequest {
    pub id: String,
    pub source_type: String,
    pub status: String,
    pub description: String,
}
