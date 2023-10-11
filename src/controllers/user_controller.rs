use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::user_entity::UserEntity;
use crate::models::user::user_error::UserError;
use crate::models::{AddResponse, FailResponse};
use crate::repositories::user::insert::insert;
use crate::AppState;

pub async fn root() -> &'static str {
    "root user"
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(request): Json<AddUserRequest>,
) -> Result<AddResponse<UserEntity>, FailResponse<UserError>> {
    match request.is_valid() {
        Ok(_) => insert(&state.db, request).await,
        Err(e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(e),
        }),
    }
}
