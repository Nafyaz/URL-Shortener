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
