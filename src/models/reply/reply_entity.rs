use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query, PgPool};

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug)]
pub struct ReplyEntity {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub post_id: Uuid,
    pub parent_reply_ids: Vec<Uuid>,
    pub child_reply_ids: Vec<Uuid>,
}

impl ReplyEntity {
    pub async fn find_with_relations(
        pool: &PgPool,
        post_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
        SELECT
            r.reply_id AS id,
            r.reply_author_id AS author_id,
            r.reply_content AS content,
            r.reply_created_at AS created_at,
            r.reply_updated_at AS updated_at,
            r.reply_post_id AS post_id,
            COALESCE(array_agg(rel.parent_reply_id), ARRAY[]::UUID[]) AS parent_reply_ids,
            COALESCE(array_agg(rel.child_reply_id), ARRAY[]::UUID[]) AS child_reply_ids
        FROM
            reply r
        LEFT JOIN
            reply_relation rel ON r.reply_id = rel.child_reply_id
        WHERE 
            r.reply_post_id = $1
        GROUP BY
            r.reply_id
        "#,
            post_id
        )
        .fetch_all(pool)
        .await?;

        let entities: Vec<ReplyEntity> = result
            .into_iter()
            .map(|row| ReplyEntity {
                id: row.id,
                author_id: row.author_id,
                content: row.content,
                created_at: row.created_at.unwrap(),
                updated_at: row.updated_at.unwrap(),
                post_id: row.post_id,
                parent_reply_ids: row.parent_reply_ids.unwrap_or_default(),
                child_reply_ids: row.child_reply_ids.unwrap_or_default(),
            })
            .collect();
        Ok(entities)
    }
}
