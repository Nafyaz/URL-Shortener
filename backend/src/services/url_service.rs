use crate::error::AppError;
use crate::error::AppError::{DatabaseQueryError, ShortCodeNotFoundError};
use crate::models::url::Url;
use crate::models::url_response::UrlResponse;
use nanoid::nanoid;
use sqlx::PgPool;
use tracing::instrument;

#[derive(Clone)]
pub struct UrlService {
    pool: PgPool,
}

impl UrlService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, url))]
    pub async fn shorten_url(&self, url: &str) -> Result<UrlResponse, AppError> {
        let short_code = nanoid!(6);

        let url = sqlx::query_as!(
            Url,
            "INSERT INTO urls (short_code, original_url) VALUES ($1, $2) RETURNING id, short_code, original_url, created_at",
            short_code,
            url
        )
            .fetch_one(&self.pool)
            .await
            .map_err(DatabaseQueryError)?;

        Ok(UrlResponse::new(&url.short_code, &url.original_url))
    }

    pub async fn get_original_url(&self, short_code: &str) -> Result<String, AppError> {
        let result = sqlx::query!(
            "SELECT original_url FROM urls WHERE short_code = $1",
            short_code
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(DatabaseQueryError)?;

        result
            .map(|record| record.original_url)
            .ok_or(ShortCodeNotFoundError(short_code.to_string()))
    }
}
