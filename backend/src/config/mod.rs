use std::env;

pub mod database;
pub mod logging;

#[derive(Clone)]
pub struct Config {
    pub backend_address: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            backend_address: env::var("BACKEND_ADDRESS").expect("BACKEND_ADDRESS must be set"),
        }
    }
}
