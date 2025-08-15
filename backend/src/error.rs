use crate::error::AppError::{
    DatabaseConnectionError, DatabaseQueryError, ShortCodeNotFoundError, UrlValidationError,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{error, warn};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database connection error")]
    DatabaseConnectionError(sqlx::Error),

    #[error("Database query error")]
    DatabaseQueryError(sqlx::Error),

    #[error("URL validation error")]
    UrlValidationError(String),

    #[error("Short Code not found")]
    ShortCodeNotFoundError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DatabaseConnectionError(err) => {
                error!("Database connection error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not connect to database".to_string(),
                )
            }
            DatabaseQueryError(err) => {
                error!("Database query error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database query failed".to_string(),
                )
            }
            UrlValidationError(url) => {
                warn!("URL validation error: {}", url);
                (
                    StatusCode::BAD_REQUEST,
                    "Could not validate URL".to_string(),
                )
            }

            ShortCodeNotFoundError(short_code) => {
                warn!("Short Code |{}| not found.", short_code);
                (
                    StatusCode::NOT_FOUND,
                    "Could not find Short Code".to_string(),
                )
            }
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
