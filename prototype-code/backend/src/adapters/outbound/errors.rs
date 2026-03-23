use crate::domain::errors::DomainError;

/// Infrastructure-level errors from the database layer.
/// These are logged here with full detail, then converted to DomainError
/// so the domain never sees database internals.
#[derive(Debug)]
pub enum DbError {
    QueryFailed(sqlx::Error),
}

impl DbError {
    pub fn into_domain(self) -> DomainError {
        match &self {
            DbError::QueryFailed(e) => tracing::error!("database query failed: {e}"),
        }
        DomainError::Internal
    }
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        DbError::QueryFailed(e)
    }
}
