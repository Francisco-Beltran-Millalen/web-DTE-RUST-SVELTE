use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

use crate::domain::errors::DomainError;

/// HTTP-level errors. Maps domain errors to HTTP status codes.
/// This is the only place that knows about both domain semantics and HTTP.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal error")]
    Internal,
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::EmailAlreadyInUse => AppError::Conflict("El email está en uso".to_string()),
            DomainError::UserNotFound => AppError::NotFound("usuario no encontrado".to_string()),
            DomainError::InvalidCredentials | DomainError::Unauthorized => AppError::Unauthorized,
            DomainError::Internal => AppError::Internal,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "no autorizado".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error interno del servidor".to_string(),
            ),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
