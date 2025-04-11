use axum::{
    extract::{Path, State},
    response::Redirect,
};
use std::sync::Arc;

use crate::error::AppError;
use crate::AppState;

pub async fn redirect_to_original(
    Path(short_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    let original_url = state.url_service.get_original_url(&short_code).await?;
    Ok(Redirect::to(&original_url))
}
