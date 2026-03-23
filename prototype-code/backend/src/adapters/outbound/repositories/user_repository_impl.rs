use crate::adapters::outbound::errors::DbError;
use crate::domain::entities::user::User;
use crate::domain::errors::DomainError;
use crate::domain::ports::outbound::user_repository::{NewUser, UserRepository};
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct UserRepositoryImpl {
    pool: SqlitePool,
}

impl UserRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        sqlx::query_as!(
            User,
            r#"SELECT
              id as "id!",
              name,
              email,
              password_hash,
              created_at as "created_at!: chrono::DateTime<chrono::Utc>",
              updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
              deleted_at as "deleted_at?: chrono::DateTime<chrono::Utc>"
              FROM user WHERE email = ?"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::from(e).into_domain())
    }

    async fn create_account(&self, new_user: NewUser) -> Result<User, DomainError> {
        let result = sqlx::query!(
            "INSERT INTO user (name, email, password_hash) VALUES (?, ?, ?)",
            new_user.name,
            new_user.email,
            new_user.password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::from(e).into_domain())?;

        let id = result.last_insert_rowid();

        sqlx::query_as!(
            User,
            r#"SELECT
              id as "id!",
              name,
              email,
              password_hash,
              created_at as "created_at!: chrono::DateTime<chrono::Utc>",
              updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
              deleted_at as "deleted_at?: chrono::DateTime<chrono::Utc>"
              FROM user WHERE id = ?"#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::from(e).into_domain())
    }
}
