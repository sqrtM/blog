use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize, PartialEq)]
pub struct PostEntity {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
