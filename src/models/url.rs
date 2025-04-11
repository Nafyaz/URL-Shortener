use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Url {
    pub id: String,
    pub original_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_url: String,
    pub original_url: String,
}

impl UrlResponse {
    pub fn new(base_url: &str, id: &str, original_url: &str) -> Self {
        Self {
            short_url: format!("{}/{}", base_url, id),
            original_url: original_url.to_string(),
        }
    }
}