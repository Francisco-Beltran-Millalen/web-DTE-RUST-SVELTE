use anyhow::{Context, Result};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::str::FromStr;

/// All configuration loaded from environment variables / .env file.
/// Call AppConfig::from_env() once at startup after dotenvy::dotenv().
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_access_token_expiry_secs: u64,
    pub jwt_refresh_token_expiry_days: u64,
    pub sii_encryption_key: String,
    pub environment: String,
    pub cors_allowed_origins: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            port: optional_parse("PORT", 3000)?,
            jwt_secret: required("JWT_SECRET")?,
            jwt_access_token_expiry_secs: optional_parse("JWT_ACCESS_TOKEN_EXPIRY_SECS", 3600)?,
            jwt_refresh_token_expiry_days: optional_parse("JWT_REFRESH_TOKEN_EXPIRY_DAYS", 7)?,
            sii_encryption_key: required("SII_ENCRYPTION_KEY")?,
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into()),
            cors_allowed_origins: std::env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:5173".into()),
        })
    }
}

/// Create the SQLite connection pool and enable foreign key enforcement.
/// Creates the database file if it does not exist.
/// PRAGMA foreign_keys = ON must run on every connection — SQLite resets it per connection.
pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

fn required(key: &str) -> Result<String> {
    std::env::var(key).with_context(|| format!("missing required env var: {key}"))
}

fn optional_parse<T: std::str::FromStr>(key: &str, default: T) -> Result<T>
where
    T::Err: std::fmt::Display,
{
    match std::env::var(key) {
        Ok(val) => val
            .parse::<T>()
            .map_err(|e| anyhow::anyhow!("invalid value for {key}: {e}")),
        Err(_) => Ok(default),
    }
}
