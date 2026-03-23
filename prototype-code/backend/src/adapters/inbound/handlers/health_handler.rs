use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::adapters::inbound::routes::AppState;

/// GET /health
/// Checks that the server is running and the database is reachable.
/// Returns 200 {"status":"ok","database":"connected"} on success.
pub async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "status": "ok", "database": "connected" })),
        ),
        Err(e) => {
            tracing::error!("health check db query failed: {e}");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "status": "error", "database": "unavailable" })),
            )
        }
    }
}
