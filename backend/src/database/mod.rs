use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;
use crate::error::AppError;
use crate::error::AppError::DatabaseConnectionError;

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<Postgres>,
}

impl DatabaseConnection {
    pub async fn new(config: &Config) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await
            .map_err(DatabaseConnectionError)?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }
}
