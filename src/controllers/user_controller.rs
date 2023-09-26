use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::add_user_response::AddUserResponse;
use crate::repositories::user::insert::insert;
use crate::AppState;

pub async fn root() -> &'static str {
    "root user"
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(request): Json<AddUserRequest>,
) -> AddUserResponse {
    match request.is_valid() {
        Ok(_) => match insert(&state.db, request).await {
            Ok(_) => AddUserResponse {
                status: StatusCode::ACCEPTED,
                message: String::from("good"),
            },
            Err(e) => AddUserResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: e.get_message(),
            },
        },
        Err(e) => AddUserResponse {
            status: StatusCode::BAD_REQUEST,
            message: e.get_message(),
        },
    }
}
