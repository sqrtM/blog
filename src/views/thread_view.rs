use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::thread::thread_entity::ThreadEntity;
use crate::views::reply_view::ReplyView;

#[derive(Serialize, Clone, Debug)]
pub struct ThreadView {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replies: Vec<ReplyView>,
    pub board_id: String,
}

impl From<ThreadEntity> for ThreadView {
    fn from(post_entity: ThreadEntity) -> Self {
        ThreadView {
            id: post_entity.id.hyphenated().to_string(),
            author_id: post_entity
                .author_id
                .unwrap_or_else(Uuid::nil)
                .hyphenated()
                .to_string(),
            author_name: "Anon".to_string(),
            title: post_entity.title,
            content: post_entity.content,
            created_at: post_entity.created_at,
            updated_at: post_entity.updated_at,
            replies: Vec::new(),
            board_id: post_entity.board_id.hyphenated().to_string(),
        }
    }
}
