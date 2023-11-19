use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Form, Json};
use uuid::Uuid;

use crate::models::reply::add_reply_to_post_request::AddReplyToPostRequest;
use crate::models::reply::reply_entity::ReplyEntity;
use crate::models::reply::reply_error::ReplyError;
use crate::models::{FailResponse, GetResponse};
use crate::AppState;
use crate::views::NewReply;
use crate::views::reply_view::ReplyView;

pub async fn get_replies_from_thread(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<GetResponse<Vec<ReplyEntity>>, FailResponse<ReplyError>> {
    match ReplyEntity::find_with_relations(&state.db, thread_id).await {
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

pub async fn add_reply(
    State(state): State<AppState>,
    Form(request): Form<AddReplyToPostRequest>,
) -> Result<NewReply, FailResponse<ReplyError>> {
    match ReplyEntity::insert(&state.db, request).await {
        Ok(entity) => Ok(NewReply {reply: ReplyView::from(entity)}),
        Err(_) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(ReplyError),
        }),
    }
}
