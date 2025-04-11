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

    pub async fn validate_url(&self, url: &str) -> Result<String, AppError> {
        // Add basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::UrlValidationError(
                "Invalid URL format. Must start with http:// or https://".to_string()
            ));
        }

        Ok(url.to_string())
    }

    pub async fn shorten_url(&self, request: UrlRequest) -> Result<UrlResponse, AppError> {
        // Validate the URL first
        let validated_url = self.validate_url(&request.url).await?;

        // Generate a unique short ID
        let id = nanoid!(6);

        // Insert into database
        let url = sqlx::query_as!(
            Url,
            "INSERT INTO urls (id, original_url) VALUES ($1, $2) RETURNING id, original_url, created_at",
            id,
            validated_url
        )
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::DatabaseQueryError)?;

        // Create and return response
        Ok(UrlResponse::new(&self.config.base_url, &url.id, &url.original_url))
    }

    pub async fn get_original_url(&self, short_id: &str) -> Result<String, AppError> {
        let result = sqlx::query!(
            "SELECT original_url FROM urls WHERE id = $1",
            short_id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::DatabaseQueryError)?;

        result
            .map(|record| record.original_url)
            .ok_or(AppError::UrlNotFoundError)
    }

    pub async fn list_urls(&self) -> Result<Vec<UrlResponse>, AppError> {
        let urls = sqlx::query_as!(
            Url,
            "SELECT id, original_url, created_at FROM urls ORDER BY created_at DESC"
        )
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseQueryError)?;

        Ok(urls
            .into_iter()
            .map(|url| UrlResponse::new(&self.config.base_url, &url.id, &url.original_url))
            .collect())
    }
}