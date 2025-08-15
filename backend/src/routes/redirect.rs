use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    response::Redirect,
};
use std::sync::Arc;
use tracing::{info, instrument};

#[instrument(skip(state))]
pub async fn redirect_to_original(
    State(state): State<Arc<AppState>>,
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError> {
    let original_url = state.url_service.get_original_url(&short_code).await?;

    info!("Redirecting to {}", original_url);
    Ok(Redirect::to(&original_url))
}
