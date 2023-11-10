use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::models::reply::add_reply_to_post_request::AddReplyToPostRequest;
use crate::models::reply::reply_entity::ReplyEntity;
use crate::models::reply::reply_error::ReplyError;
use crate::models::{AddResponse, FailResponse};
use crate::repositories::replies::insert::insert_reply_to_post;
use crate::AppState;

pub async fn add_reply_to_post(
    State(state): State<AppState>,
    Json(request): Json<AddReplyToPostRequest>,
) -> Result<AddResponse<ReplyEntity>, FailResponse<ReplyError>> {
    match request.is_valid() {
        Ok(_) => insert_reply_to_post(&state.db, request).await,
        Err(_e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(ReplyError),
        }),
    }
}
