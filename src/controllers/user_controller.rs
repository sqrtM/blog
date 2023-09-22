use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::AppState;
use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::add_user_response::AddUserResponse;
use crate::repositories::user::add::add;

pub async fn root() -> &'static str {
    "root user_entity"
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(request): Json<AddUserRequest>,
) -> AddUserResponse {
    if request.is_valid() {
        match add(&state.db, request).await {
            Ok(_) => AddUserResponse { status: StatusCode::ACCEPTED },
            Err(_) => AddUserResponse { status: StatusCode::GATEWAY_TIMEOUT }
        }
    } else {
        AddUserResponse { status: StatusCode::ALREADY_REPORTED }
    }
}