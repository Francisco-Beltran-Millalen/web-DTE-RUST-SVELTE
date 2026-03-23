use axum::{routing::{get, post}, Router};
use sqlx::SqlitePool;
use std::sync::Arc;

use super::handlers::health_handler::health_handler;
use super::handlers::user_handler::create_account_handler;
use crate::domain::ports::inbound::create_account_use_case::CreateAccountUseCase;

/// Application state shared across all handlers via Axum's State extractor.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub create_account_use_case: Arc<dyn CreateAccountUseCase + Send + Sync>,
}

/// Build the Axum Router with all routes registered.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/auth/register", post(create_account_handler))
        .with_state(state)
}
