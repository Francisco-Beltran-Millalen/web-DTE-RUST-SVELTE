use anyhow::Result;
use std::sync::Arc;
use dte_auto_upload::adapters::inbound::routes::{build_router, AppState};
use dte_auto_upload::adapters::outbound::repositories::user_repository_impl::UserRepositoryImpl;
use dte_auto_upload::config::{AppConfig, create_pool};
use dte_auto_upload::domain::services::create_account_use_case_impl::CreateAccountUseCaseImpl;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file (silently ignored if not present — production uses real env vars)
    let _ = dotenvy::dotenv();

    // Tracing / logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from env
    let config = AppConfig::from_env()?;

    tracing::info!(environment = %config.environment, "starting DTE Auto-Upload backend");

    // Database pool
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("database connected");

    // Run migrations automatically (no sqlx-cli required)
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("migrations applied");

    // Wire dependencies
    let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let create_account_use_case = Arc::new(CreateAccountUseCaseImpl::new(user_repository));

    // Wire app state
    let state = AppState {
        pool,
        create_account_use_case,
    };

    // Build router
    let app = build_router(state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
