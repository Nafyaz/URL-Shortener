use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_address: String,
    pub base_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_address: env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set"),
            base_url: env::var("BASE_URL").expect("BASE_URL must be set"),
        }
    }
}
