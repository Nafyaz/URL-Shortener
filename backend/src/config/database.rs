use crate::error::AppError;
use crate::error::AppError::DatabaseConnectionError;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:password@localhost:5432/ecommerce".to_string()
            }),
            max_connections: 10,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Duration::from_secs(1800), // 30 minutes
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),

            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a valid number"),

            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("DB_MIN_CONNECTIONS must be a valid number"),

            acquire_timeout: Duration::from_secs(
                env::var("DB_ACQUIRE_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .expect("DB_ACQUIRE_TIMEOUT must be a valid number"),
            ),

            idle_timeout: Duration::from_secs(
                env::var("DB_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .expect("DB_IDLE_TIMEOUT must be a valid number"),
            ),

            max_lifetime: Duration::from_secs(
                env::var("DB_MAX_LIFETIME")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()
                    .expect("DB_MAX_LIFETIME must be a valid number"),
            ),
        }
    }

    pub async fn create_pool(&self) -> Result<PgPool, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(self.acquire_timeout)
            .idle_timeout(self.idle_timeout)
            .max_lifetime(self.max_lifetime)
            .connect(&self.url)
            .await
            .map_err(DatabaseConnectionError)?;

        tracing::info!(
            "Database connection pool created with {} max connections",
            self.max_connections
        );

        Ok(pool)
    }

    // TODO: Do I need it?
    // pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let pool = self.create_pool().await?;
    //
    //     sqlx::query("SELECT 1").fetch_one(&pool).await?;
    //
    //     tracing::info!("Database connection test successful");
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert!(config.url.contains("postgresql://"));
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 1);
    }

    #[test]
    fn test_database_config_from_env() {
        env::set_var("DATABASE_URL", "postgresql://test:test@localhost:5432/test");
        env::set_var("DB_MAX_CONNECTIONS", "5");

        let config = DatabaseConfig::from_env();
        assert_eq!(config.url, "postgresql://test:test@localhost:5432/test");
        assert_eq!(config.max_connections, 5);

        env::remove_var("DATABASE_URL");
        env::remove_var("DB_MAX_CONNECTIONS");
    }
}
