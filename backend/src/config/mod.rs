use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub backend_address: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            backend_address: env::var("BACKEND_ADDRESS").expect("BACKEND_ADDRESS must be set"),
        }
    }
}
