use thiserror::Error;

/// Business rule violations and domain-level failures.
/// These are the only error types that cross the domain boundary.
/// Infrastructure errors (DB, network) are logged at the adapter level
/// and converted into one of these variants before reaching the domain.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("email already in use")]
    EmailAlreadyInUse,

    #[error("user not found")]
    UserNotFound,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("unauthorized")]
    Unauthorized,

    /// An infrastructure failure occurred. Details were logged at the adapter boundary.
    #[error("internal error")]
    Internal,
}
