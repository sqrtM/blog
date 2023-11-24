use std::collections::HashSet;

use chrono::{DateTime, Utc};
use regex::Regex;
use sanitize_html::rules::predefined::DEFAULT;
use sanitize_html::sanitize_str;
use serde::Serialize;
use sqlx::{PgPool, query_as};
use sqlx::types::Uuid;

use crate::models::reply::add_reply_to_thread_request::AddReplyToThreadRequest;

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug)]
pub struct ReplyEntity {
    #[sqlx(rename = "reply_id")]
    pub id: Uuid,
    #[sqlx(rename = "reply_author_id")]
    pub author_id: Option<Uuid>,
    #[sqlx(rename = "reply_content")]
    pub content: String,
    #[sqlx(rename = "reply_created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "reply_updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "reply_post_id")]
    pub post_id: Uuid,
    pub parent_reply_ids: Option<Vec<Option<Uuid>>>,
    pub child_reply_ids: Option<Vec<Option<Uuid>>>,
}

impl ReplyEntity {
    pub async fn find_with_relations(
        pool: &PgPool,
        post_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let result = query_as::<_, ReplyEntity>(
            //language=PostgreSQL
            r#"
            SELECT r.reply_id,
                   r.reply_author_id,
                   r.reply_content,
                   r.reply_created_at,
                   r.reply_updated_at,
                   r.reply_post_id,
                   COALESCE(array_agg(parent.parent_reply_id), ARRAY[]::UUID[]) AS parent_reply_ids,
                   COALESCE(array_agg(child.child_reply_id), ARRAY[]::UUID[]) AS child_reply_ids
            FROM reply r
            LEFT JOIN reply_relation parent ON r.reply_id = parent.child_reply_id
            LEFT JOIN reply_relation child ON r.reply_id = child.parent_reply_id
            WHERE r.reply_post_id = $1
            GROUP BY r.reply_id;
        "#,
        )
            .bind(post_id)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    /// On insert, content is sanitized of HTML tags.
    /// Then, HTML tags which are allowed are deliberately
    /// reinserted when serving the HTML to the user.
    pub async fn insert(
        pool: &PgPool,
        request: AddReplyToThreadRequest,
        thread_id: Uuid,
    ) -> Result<ReplyEntity, sqlx::Error> {
        let referenced_reply_ids: HashSet<Uuid> =
            Regex::new(r#">>([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})"#)
                .unwrap()
                .captures_iter(&request.content)
                .filter_map(|cap| Uuid::parse_str(&cap[1]).ok())
                .collect();

        let new_reply_id = Uuid::new_v4();
        let content = sanitize_str(&DEFAULT, &request.content).unwrap();
        let reply: ReplyEntity = sqlx::query_as::<_, ReplyEntity>(
            //language=PostgreSQL
            "INSERT INTO reply (reply_id, reply_author_id, reply_content, reply_post_id)
                VALUES ($1, $2, $3, $4)
                RETURNING reply_id,
                    reply_author_id,
                    reply_content,
                    reply_created_at,
                    reply_updated_at,
                    reply_post_id,
                    ARRAY []::UUID[] AS parent_reply_ids,
                    ARRAY []::UUID[]  AS child_reply_ids",
        )
            .bind(new_reply_id)
            .bind(request.author_id)
            .bind(content)
            .bind(thread_id)
            .fetch_one(pool)
            .await?;

        for referenced_reply_id in referenced_reply_ids {
            sqlx::query(
                //language=PostgreSQL
                "INSERT INTO 
                    reply_relation (parent_reply_id, child_reply_id)
                VALUES 
                    ($1, $2)",
            )
                .bind(referenced_reply_id)
                .bind(new_reply_id)
                .execute(pool)
                .await?;
        }
        Ok(reply)
    }
}
