use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::user_entity::UserEntity;
use crate::models::user::user_error::UserError;
use crate::models::AddResponse;
use crate::repositories::user::insert::insert;
use crate::AppState;

pub async fn root() -> &'static str {
    "root user"
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(request): Json<AddUserRequest>,
) -> Result<AddResponse<UserEntity>, AddResponse<UserError>> {
    match request.is_valid() {
        Ok(_) => match insert(&state.db, request).await {
            Ok(u) => Ok(AddResponse {
                status: StatusCode::ACCEPTED,
                message: Json(u),
            }),
            Err(e) => Err(AddResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: Json(e),
            }),
        },
        Err(e) => Err(AddResponse {
            status: StatusCode::BAD_REQUEST,
            message: Json(e),
        }),
    }
}
