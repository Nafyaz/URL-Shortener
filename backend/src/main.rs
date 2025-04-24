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
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use config::AppConfig;
use database::DatabaseConnection;
use routes::redirect::redirect_to_original;
use routes::shorten::shorten_url;
use services::url_service::UrlService;

#[derive(Clone)]
pub struct AppState {
    url_service: UrlService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new();
    let db_conn = DatabaseConnection::new(&config).await?;
    let url_service = UrlService::new(db_conn.get_pool(), config.clone());

    let app_state = Arc::new(AppState { url_service });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/{short_code}", get(redirect_to_original))
        .with_state(app_state)
        .layer(cors);

    let addr = config.server_address.parse::<SocketAddr>()?;
    println!("Server running on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
