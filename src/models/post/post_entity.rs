use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query, PgPool};

#[derive(sqlx::FromRow, Serialize, PartialEq)]
pub struct PostEntity {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostEntity {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
        SELECT * FROM posts;
        "#
        )
        .fetch_all(pool)
        .await?;

        let posts: Vec<PostEntity> = result
            .into_iter()
            .map(|row| PostEntity {
                id: row.post_id,
                author_id: row.post_author_id,
                title: row.post_title,
                content: row.post_content,
                created_at: row.post_created_at.unwrap(),
                updated_at: row.post_updated_at.unwrap(),
            })
            .collect::<Vec<PostEntity>>();

        Ok(posts)
    }
}
