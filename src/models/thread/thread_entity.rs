use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query, PgPool};

#[derive(sqlx::FromRow, Serialize, PartialEq)]
pub struct ThreadEntity {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ThreadEntity {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
        SELECT * FROM thread;
        "#
        )
        .fetch_all(pool)
        .await?;

        let threads: Vec<ThreadEntity> = result
            .into_iter()
            .map(|row| ThreadEntity {
                id: row.thread_id,
                author_id: row.thread_author_id,
                title: row.thread_title,
                content: row.thread_content,
                created_at: row.thread_created_at.unwrap(),
                updated_at: row.thread_updated_at.unwrap(),
            })
            .collect::<Vec<ThreadEntity>>();

        Ok(threads)
    }
}
