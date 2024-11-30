use crate::error::{Error, Result};
use sqlx::postgres::PgConnectOptions;

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self> {
        let username =
            std::env::var("NEON_USER").map_err(|_| Error::validation("NEON_USER must be set"))?;
        let password = std::env::var("NEON_PASSWORD")
            .map_err(|_| Error::validation("NEON_PASSWORD must be set"))?;

        Ok(Self {
            host: "ep-dry-thunder-a2bjpu75-pooler.eu-central-1.aws.neon.tech".to_string(),
            port: 5432,
            username,
            password,
            database_name: "neondb".to_string(),
        })
    }

    pub fn connection_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .database(&self.database_name)
            .ssl_mode(sqlx::postgres::PgSslMode::Require)
            .statement_cache_capacity(100)
            .application_name("bustleit-lambda")
    }
}
