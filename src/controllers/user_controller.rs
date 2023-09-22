use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::AppState;
use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::add_user_response::AddUserResponse;
use crate::models::user::user_error::{InvalidPasswordReason, UserError};
use crate::repositories::user::insert::insert;

pub async fn root() -> &'static str {
    "root user_entity"
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(request): Json<AddUserRequest>,
) -> AddUserResponse {
    if request.is_valid() {
        match insert(&state.db, request).await {
            Ok(_) => AddUserResponse {
                status: StatusCode::ACCEPTED,
                message: String::from("good"),
            },
            Err(e) => AddUserResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.get_message(),
            }
        }
    } else {
        AddUserResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: UserError::PasswordInvalid(InvalidPasswordReason::LessThanEightCharacters).get_message(),
        }
    }
}