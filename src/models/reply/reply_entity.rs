use std::collections::HashSet;

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query_as, PgPool};

use crate::models::reply::add_reply_to_thread_request::AddReplyToThreadRequest;

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug)]
pub struct ReplyEntity {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
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
            SELECT r.reply_id                                                 AS id,
                   r.reply_author_id                                          AS author_id,
                   r.reply_content                                            AS content,
                   r.reply_created_at                                         AS created_at,
                   r.reply_updated_at                                         AS updated_at,
                   r.reply_post_id                                            AS post_id,
                   COALESCE(array_agg(rel.parent_reply_id), ARRAY []::UUID[]) AS parent_reply_ids,
                   COALESCE(array_agg(rel.child_reply_id), ARRAY []::UUID[])  AS child_reply_ids
            FROM reply r
                     LEFT JOIN
                 reply_relation rel ON r.reply_id = rel.child_reply_id
            WHERE r.reply_post_id = $1
            GROUP BY r.reply_id
        "#,
        )
        .bind(post_id)
        .fetch_all(pool)
        .await?;

        println!("{:?}", result);
        Ok(result)
    }

    pub async fn insert(
        pool: &PgPool,
        request: AddReplyToThreadRequest,
        thread_id: Uuid,
    ) -> Result<ReplyEntity, sqlx::Error> {
        let referenced_reply_ids: HashSet<Uuid> = Regex::new(r#">>([a-fA-F0-9-]+)"#)
            .unwrap()
            .captures_iter(&request.content)
            .filter_map(|cap| Uuid::parse_str(&cap[1]).ok())
            .collect();

        let new_reply_id = Uuid::new_v4();
        let reply: ReplyEntity = sqlx::query_as::<_, ReplyEntity>(
            "INSERT INTO reply (reply_id, reply_author_id, reply_content, reply_post_id)
                VALUES ($1, $2, $3, $4)
                RETURNING reply_id AS id,
                    reply_author_id AS author_id,
                    reply_content AS content,
                    reply_created_at AS created_at,
                    reply_updated_at AS updated_at,
                    reply_post_id AS post_id,
                    ARRAY []::UUID[] AS parent_reply_ids,
                    ARRAY []::UUID[]  AS child_reply_ids",
        )
        .bind(new_reply_id)
        .bind(request.author_id)
        .bind(request.content)
        .bind(thread_id)
        .fetch_one(pool)
        .await?;

        for referenced_reply_id in referenced_reply_ids {
            sqlx::query!(
                "INSERT INTO reply_relation (parent_reply_id, child_reply_id)
             VALUES ($1, $2)",
                referenced_reply_id,
                new_reply_id
            )
            .execute(pool)
            .await?;
        }
        Ok(reply)
    }
}
