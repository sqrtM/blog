use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::post::post_entity::PostEntity;
use crate::views::reply_view::ReplyView;

#[derive(Serialize)]
pub struct PostView {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replies: Vec<ReplyView>,
}

impl From<PostEntity> for PostView {
    fn from(post_entity: PostEntity) -> Self {
        PostView {
            id: post_entity.id,
            author_id: post_entity.author_id,
            author_name: None,
            title: post_entity.title,
            content: post_entity.content,
            created_at: post_entity.created_at,
            updated_at: post_entity.updated_at,
            replies: Vec::new(),
        }
    }
}
