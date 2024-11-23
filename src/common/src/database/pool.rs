use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use super::config::DatabaseConfig;
use crate::error::{Error, Result};

pub async fn create_pool(config: DatabaseConfig) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(config.connection_options())
        .await
        .map_err(Error::from)
}
