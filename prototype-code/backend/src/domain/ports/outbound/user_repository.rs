use crate::domain::entities::user::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn create_account(&self, new_user: NewUser) -> Result<User, DomainError>;
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
