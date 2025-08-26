use crate::error::AppError;
use crate::error::AppError::UrlValidationError;
use crate::models::url_request::UrlRequest;
use crate::models::url_response::UrlResponse;
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use tracing::{info, instrument};

// TODO: Do I / Should I really need to validate the URL like this?
fn validate_url(url: &str) -> Result<String, AppError> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(UrlValidationError(url.to_string()));
    }

    Ok(url.to_string())
}

#[instrument(skip_all)]
pub async fn shorten_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UrlRequest>,
) -> Result<(StatusCode, Json<UrlResponse>), AppError> {
    let url = validate_url(&payload.url)?;
    let result = state.url_service.get_short_code(&url).await?;

    info!("Shortened URL: {}", result.short_url);
    Ok((StatusCode::CREATED, Json(result)))
}
