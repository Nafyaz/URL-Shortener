use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_address: String,
    pub base_url: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost/urlshortener".to_string()
            }),
            server_address: env::var("SERVER_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }
}
