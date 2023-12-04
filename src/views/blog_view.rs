use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::blog::blog_entity::{BlogEntity, BlogSummaryEntity};

#[derive(Serialize, Clone, Debug)]
pub struct BlogSummaryView {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BlogPostView {
    pub id: String,
    pub title: String,
    pub content: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl From<BlogSummaryEntity> for BlogSummaryView {
    fn from(blog_entity: BlogSummaryEntity) -> Self {
        Self {
            id: blog_entity.id.hyphenated().to_string(),
            title: blog_entity.title,
            description: blog_entity.description,
            created_at: blog_entity.created_at,
        }
    }
}

impl From<BlogEntity> for BlogPostView {
    fn from(blog_entity: BlogEntity) -> Self {
        Self {
            id: blog_entity.id.hyphenated().to_string(),
            title: blog_entity.title,
            content: blog_entity.content,
            description: blog_entity.description,
            created_at: blog_entity.created_at,
        }
    }
}
