use nanoid::nanoid;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::models::url::{Url, UrlRequest, UrlResponse};

#[derive(Clone)]
pub struct UrlService {
    pool: PgPool,
    config: AppConfig,
}

impl UrlService {
    pub fn new(pool: PgPool, config: AppConfig) -> Self {
        Self { pool, config }
    }

    async fn validate_url(&self, url: &str) -> Result<String, AppError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::UrlValidationError(
                "Invalid URL format. Must start with http:// or https://".to_string(),
            ));
        }

        Ok(url.to_string())
    }

    pub async fn shorten_url(&self, request: UrlRequest) -> Result<UrlResponse, AppError> {
        let validated_url = self.validate_url(&request.url).await?;

        let short_code = nanoid!(6);

        let url = sqlx::query_as!(
            Url,
            "INSERT INTO urls (short_code, original_url) VALUES ($1, $2) RETURNING id, short_code, original_url, created_at",
            short_code,
            validated_url
        )
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::DatabaseQueryError)?;

        Ok(UrlResponse::new(
            &self.config.base_url,
            &url.short_code,
            &url.original_url,
        ))
    }

    pub async fn get_original_url(&self, short_code: &str) -> Result<String, AppError> {
        let result = sqlx::query!(
            "SELECT original_url FROM urls WHERE short_code = $1",
            short_code
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseQueryError)?;

        result
            .map(|record| record.original_url)
            .ok_or(AppError::UrlNotFoundError)
    }
}
