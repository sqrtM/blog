use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use crate::models::user::user_entity::UserEntity;

#[derive(sqlx::FromRow)]
pub struct PostEntity {
    #[sqlx(rename = "post_id")]
    pub id: Uuid,
    #[sqlx(flatten)]
    pub author: UserEntity,
    #[sqlx(rename = "post_title")]
    pub title: String,
    #[sqlx(rename = "post_content")]
    pub content: String,
    #[sqlx(rename = "post_created_at")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "post_updated_at")]
    pub updated_at: DateTime<Utc>,
}
