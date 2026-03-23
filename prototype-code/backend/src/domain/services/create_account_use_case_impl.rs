use crate::domain::entities::user::User;
use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::create_account_use_case::CreateAccountUseCase;
use crate::domain::ports::outbound::user_repository::{NewUser, UserRepository};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateAccountUseCaseImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl CreateAccountUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl CreateAccountUseCase for CreateAccountUseCaseImpl {
    async fn execute(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, DomainError> {
        let existing = self.user_repository.find_user_by_email(&email).await?;
        if existing.is_some() {
            return Err(DomainError::EmailAlreadyInUse);
        }

        let hashed_password = hash_password(password)?;

        let new_user = NewUser {
            name,
            email,
            password_hash: hashed_password,
        };

        self.user_repository.create_account(new_user).await
    }
}

fn hash_password(unhashed_password: String) -> Result<String, DomainError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(unhashed_password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("password hashing failed: {e}");
            DomainError::Internal
        })
        .map(|hash| hash.to_string())
}
