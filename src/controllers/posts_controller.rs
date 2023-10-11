use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::post::add_post_request::AddPostRequest;
use crate::models::post::post_entity::PostEntity;
use crate::models::post::post_error::PostError;
use crate::models::{AddResponse, FailResponse};
use crate::repositories::posts::insert::insert;
use crate::AppState;

pub async fn add_post(
    State(state): State<AppState>,
    Json(request): Json<AddPostRequest>,
) -> Result<AddResponse<PostEntity>, FailResponse<PostError>> {
    match request.is_valid() {
        Ok(_) => insert(&state.db, request).await,
        Err(_e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(PostError),
        }),
    }
}
