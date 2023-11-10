use crate::models::reply::reply_entity::ReplyEntity;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ReplyView {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>, // You might want to fetch author_name separately
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub parent_reply_ids: Vec<Uuid>,
    pub child_reply_ids: Vec<Uuid>,
}

impl From<ReplyEntity> for ReplyView {
    fn from(reply_entity: ReplyEntity) -> Self {
        ReplyView {
            id: reply_entity.id,
            author_id: reply_entity.author_id,
            author_name: None,
            content: reply_entity.content,
            created_at: reply_entity.created_at,
            updated_at: reply_entity.updated_at,
            parent_reply_ids: reply_entity.parent_reply_ids,
            child_reply_ids: reply_entity.child_reply_ids,
        }
    }
}
