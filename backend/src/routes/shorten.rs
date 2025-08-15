use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::models::url::UrlRequest;
use crate::services::url_service::UrlService;
use crate::AppState;

pub async fn shorten_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UrlRequest>,
) -> impl IntoResponse {
    let result = state.url_service.shorten_url(payload).await;

    match result {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(e) => e.into_response(),
    }
}
