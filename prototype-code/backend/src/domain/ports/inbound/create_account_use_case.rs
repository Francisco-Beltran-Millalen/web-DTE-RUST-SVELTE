use crate::domain::entities::user::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait CreateAccountUseCase {
    async fn execute(&self, name: String, email: String, password: String) -> Result<User, DomainError>;
}
