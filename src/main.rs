use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

// Define application state
#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

// URL request model
#[derive(Deserialize)]
struct UrlRequest {
    url: String,
}

// URL response model
#[derive(Serialize)]
struct UrlResponse {
    short_url: String,
    original_url: String,
}

// URL model for database
#[derive(Serialize, Deserialize)]
struct Url {
    id: String,
    original_url: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database connection string
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/urlshortener".to_string());

    // Set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create the table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            original_url TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    )
        .execute(&pool)
        .await?;

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Set up state
    let state = Arc::new(AppState { pool });

    // Build our application with routes
    let app = Router::new()
        .route("/api/shorten", post(shorten_url))
        .route("/api/urls", get(get_all_urls))
        .route("/:id", get(redirect_to_original))
        .with_state(state)
        .layer(cors);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Handler to shorten a URL
async fn shorten_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UrlRequest>,
) -> impl IntoResponse {
    // Validate URL
    if !payload.url.starts_with("http://") && !payload.url.starts_with("https://") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid URL format. URL must start with http:// or https://"
            })),
        );
    }

    // Generate a short ID using nanoid (6 characters)
    let id = nanoid!(6);

    // Store in database
    let result = sqlx::query(
        "INSERT INTO urls (id, original_url, created_at) VALUES ($1, $2, $3)",
    )
        .bind(&id)
        .bind(&payload.url)
        .bind(chrono::Utc::now())
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => {
            let response = UrlResponse {
                short_url: format!("http://localhost:3000/{}", id),
                original_url: payload.url,
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to shorten URL"
                })),
            )
        }
    }
}

// Handler to get all URLs
async fn get_all_urls(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Url,
        "SELECT id, original_url, created_at FROM urls ORDER BY created_at DESC"
    )
        .fetch_all(&state.pool)
        .await;

    match result {
        Ok(urls) => {
            let urls_with_short = urls
                .into_iter()
                .map(|url| UrlResponse {
                    short_url: format!("http://localhost:3000/{}", url.id),
                    original_url: url.original_url,
                })
                .collect::<Vec<_>>();
            (StatusCode::OK, Json(urls_with_short))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to retrieve URLs"
                })),
            )
        }
    }
}

// Handler to redirect to original URL
async fn redirect_to_original(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "SELECT original_url FROM urls WHERE id = $1",
        id
    )
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(Some(record)) => {
            let original_url = record.original_url;
            Redirect::to(&original_url).into_response()
        }
        Ok(None) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/html".parse().unwrap());
            (
                StatusCode::NOT_FOUND,
                headers,
                "<h1>404 - URL not found</h1>",
            )
                .into_response()
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response()
        }
    }
}