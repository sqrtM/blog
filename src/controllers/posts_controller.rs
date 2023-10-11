use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::Error;

use crate::models::post::add_post_request::AddPostRequest;
use crate::models::post::post_entity::PostEntity;
use crate::models::post::post_error::PostError;
use crate::models::AddResponse;
use crate::repositories::posts::insert::insert;
use crate::AppState;

pub async fn add_post(
    State(state): State<AppState>,
    Json(request): Json<AddPostRequest>,
) -> Result<AddResponse<PostEntity>, AddResponse<PostError>> {
    match request.is_valid() {
        Ok(_) => match insert(&state.db, request).await {
            Ok(p) => Ok(AddResponse {
                status: StatusCode::ACCEPTED,
                message: Json(p),
            }),
            Err(_e) => Err(AddResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: Json(PostError),
            }),
        },
        Err(_e) => Err(AddResponse {
            status: StatusCode::BAD_REQUEST,
            message: Json(PostError),
        }),
    }
}
