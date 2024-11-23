use sqlx::postgres::PgConnectOptions;

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            host: std::env::var("NEON_HOST").map_err(|_| "NEON_HOST must be set")?,
            port: std::env::var("NEON_PORT")
                .map_err(|_| "NEON_PORT must be set")?
                .parse()
                .map_err(|_| "NEON_PORT must be a valid port number")?,
            username: std::env::var("NEON_USER").map_err(|_| "NEON_USER must be set")?,
            password: std::env::var("NEON_PASSWORD").map_err(|_| "NEON_PASSWORD must be set")?,
            database_name: std::env::var("NEON_DB").map_err(|_| "NEON_DB must be set")?,
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
    }
}
