use std::sync::Arc;
use axum_test::TestServer;
use dte_auto_upload::adapters::inbound::routes::{build_router, AppState};
use dte_auto_upload::adapters::outbound::repositories::user_repository_impl::UserRepositoryImpl;
use dte_auto_upload::config::create_pool;
use dte_auto_upload::domain::services::create_account_use_case_impl::CreateAccountUseCaseImpl;

/// Spin up a test server backed by an in-memory SQLite database.
async fn test_server() -> TestServer {
    let pool = create_pool("sqlite::memory:")
        .await
        .expect("failed to create in-memory SQLite pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let create_account_use_case = Arc::new(CreateAccountUseCaseImpl::new(user_repository));

    let state = AppState {
        pool,
        create_account_use_case,
    };

    let app = build_router(state);
    TestServer::new(app)
}

#[tokio::test]
async fn health_returns_200_with_database_connected() {
    let server = test_server().await;

    let response = server.get("/health").await;

    response.assert_status_ok();
    response.assert_json_contains(&serde_json::json!({
        "status": "ok",
        "database": "connected"
    }));
}
