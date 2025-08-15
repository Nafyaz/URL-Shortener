use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Url {
    pub id: Uuid,
    pub short_code: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
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
    pub fn new(base_url: &str, short_code: &str, original_url: &str) -> Self {
        Self {
            short_url: format!("{}/{}", base_url, short_code),
            original_url: original_url.to_string(),
        }
    }
}
