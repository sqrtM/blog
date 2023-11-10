use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::change_password_request::ChangePasswordRequest;
use crate::models::user::user_entity::UserEntity;
use crate::models::user::user_error::UserError;
use crate::models::{AddResponse, FailResponse};
use crate::AppState;

pub async fn root() -> &'static str {
    "root user"
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(request): Json<AddUserRequest>,
) -> Result<AddResponse<String>, FailResponse<UserError>> {
    match request.is_valid() {
        Ok(_) => UserEntity::insert(&state.db, request).await,
        Err(e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(e),
        }),
    }
}

pub async fn change_user_password(
    State(state): State<AppState>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<AddResponse<String>, FailResponse<UserError>> {
    match request.is_valid() {
        Ok(_) => UserEntity::change_password(&state.db, request).await,
        Err(_) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(UserError::RecoveryKeyInvalid),
        }),
    }
}
