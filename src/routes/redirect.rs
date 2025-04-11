use axum::{
    extract::{Path, State},
    response::Redirect,
};
use std::sync::Arc;

use crate::AppState;
use crate::error::AppError;

pub async fn redirect_to_original(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    let original_url = state.url_service.get_original_url(&id).await?;
    Ok(Redirect::to(&original_url))
}