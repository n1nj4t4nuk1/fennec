use serde::Deserialize;

/// JSON request body for `PUT /sources/{id}`.
#[derive(Deserialize)]
pub struct UpdateSourceRequest {
    pub status: String,
    pub description: String,
}
