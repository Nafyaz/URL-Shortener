mod config;
mod database;
mod error;
mod models;
mod routes;
mod services;
mod telemetry;

use crate::telemetry::init_telemetry;
use axum::http::{header, HeaderValue, Method};
use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use database::DatabaseConnection;
use routes::redirect::redirect_to_original;
use routes::shorten::shorten_url;
use services::url_service::UrlService;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    url_service: UrlService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    init_telemetry();

    let db_conn = DatabaseConnection::new(&config).await?;
    let url_service = UrlService::new(db_conn.get_pool());

    let app_state = Arc::new(AppState { url_service });

    // TODO: Move these to middleware
    // TODO: Learn more about CorsLayer and Fix CORS
    // let allowed_origins = ["https://my-frontend.com".parse::<HeaderValue>().unwrap()];
    // let cors_layer = CorsLayer::new()
    //     .allow_origin(allowed_origins)
    //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //     .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
    //     .allow_credentials(true);

    let router = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/{short_code}", get(redirect_to_original))
        .with_state(app_state);
    // .layer(
    //     ServiceBuilder::new()
    //         .layer(TraceLayer::new_for_http())
    //         .layer(cors_layer),
    // );

    let socket_addr = config.backend_address.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(socket_addr).await?;

    info!("Server running on {}", socket_addr);
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
