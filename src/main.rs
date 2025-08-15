mod config;
mod database;
mod error;
mod models;
mod routes;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use config::AppConfig;
use database::DatabaseConnection;
use routes::shorten::{shorten_url, list_urls};
use routes::redirect::redirect_to_original;
use services::url_service::UrlService;

#[derive(Clone)]
pub struct AppState {
    url_service: UrlService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::new();

    // Set up database connection
    let db_conn = DatabaseConnection::new(&config).await?;

    // Create URL service
    let url_service = UrlService::new(db_conn.get_pool(), config.clone());

    // Create application state
    let state = Arc::new(AppState { url_service });

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/api/shorten", post(shorten_url))
        .route("/api/urls", get(list_urls))
        .route("/:id", get(redirect_to_original))
        .with_state(state)
        .layer(cors);

    // Run the server
    let addr = config.server_address.parse::<SocketAddr>()?;
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}