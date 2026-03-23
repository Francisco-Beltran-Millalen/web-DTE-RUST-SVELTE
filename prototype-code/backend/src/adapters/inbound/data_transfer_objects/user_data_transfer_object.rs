use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAccountRequestDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: i64,
    pub name: String,
    pub email: String,
}
