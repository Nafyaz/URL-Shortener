use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database connection error")]
    DatabaseConnectionError(#[source] sqlx::Error),

    #[error("URL validation error")]
    UrlValidationError(String),

    #[error("Database query error")]
    DatabaseQueryError(#[source] sqlx::Error),

    #[error("URL not found")]
    UrlNotFoundError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseConnectionError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not connect to database".to_string(),
            ),
            AppError::UrlValidationError(details) => (StatusCode::BAD_REQUEST, details),
            AppError::DatabaseQueryError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database query failed".to_string(),
            ),
            AppError::UrlNotFoundError => (StatusCode::NOT_FOUND, "URL not found".to_string()),
        };

        (
            status,
            Json(json!({
                "error": message
            })),
        )
            .into_response()
    }
}
