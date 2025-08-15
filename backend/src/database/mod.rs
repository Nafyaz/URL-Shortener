use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::AppConfig;
use crate::error::AppError;

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<Postgres>,
}

impl DatabaseConnection {
    pub async fn new(config: &AppConfig) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await
            .map_err(AppError::DatabaseConnectionError)?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }
}
