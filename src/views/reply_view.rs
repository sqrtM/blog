use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::reply::reply_entity::ReplyEntity;

#[derive(Serialize, Clone, Debug)]
pub struct ReplyView {
    pub id: String,
    pub author_id: String,
    pub author_name: String, // You might want to fetch author_name separately
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub parent_reply_ids: Vec<Uuid>,
    pub child_reply_ids: Vec<Uuid>,
}

impl From<ReplyEntity> for ReplyView {
    fn from(reply_entity: ReplyEntity) -> Self {
        ReplyView {
            id: reply_entity.id.hyphenated().to_string(),
            author_id: reply_entity
                .author_id
                .unwrap_or_else(|| Uuid::nil())
                .hyphenated()
                .to_string(),
            author_name: "Anon".to_string(),
            content: reply_entity.content,
            created_at: reply_entity.created_at,
            updated_at: reply_entity.updated_at,
            parent_reply_ids: reply_entity.parent_reply_ids,
            child_reply_ids: reply_entity.child_reply_ids,
        }
    }
}
