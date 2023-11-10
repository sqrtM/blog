use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::models::reply::reply_entity::ReplyEntity;
use crate::models::reply::reply_error::ReplyError;
use crate::models::{FailResponse, GetResponse};
use crate::AppState;

pub async fn get_replies_from_post(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<GetResponse<Vec<ReplyEntity>>, FailResponse<ReplyError>> {
    match ReplyEntity::find_with_relations(&state.db, post_id).await {
        Ok(entity) => Ok(GetResponse {
            status: StatusCode::ACCEPTED,
            content: Json(entity),
        }),
        Err(_) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(ReplyError),
        }),
    }
}
