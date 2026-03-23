use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::adapters::inbound::data_transfer_objects::user_data_transfer_object::{
    CreateAccountRequestDto, UserResponseDto,
};
use crate::adapters::inbound::errors::AppError;
use crate::adapters::inbound::routes::AppState;

/// POST /auth/register
/// Creates a new user account.
/// Returns 201 with the created user (id, name, email) on success.
pub async fn create_account_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateAccountRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let user = state
        .create_account_use_case
        .execute(body.name, body.email, body.password)
        .await?;

    let response = UserResponseDto {
        id: user.id,
        name: user.name,
        email: user.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
