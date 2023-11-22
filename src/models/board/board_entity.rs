use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug)]
pub struct BoardEntity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub authorized_only: bool,
}

#[derive(sqlx::FromRow, Serialize, PartialEq, Debug)]
pub struct BoardEntityWithThreadInfo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub authorized_only: bool,
    pub total_threads: i64,
    pub most_recent_post_time: DateTime<Utc>,
    pub most_recent_post_title: String,
}

impl BoardEntity {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
        SELECT 
            board_id, 
            board_name, 
            board_description, 
            board_authorized_only
        FROM 
            board
        "#
        )
        .fetch_all(pool)
        .await?;

        let boards: Vec<Self> = result
            .into_iter()
            .map(|row| Self {
                id: row.board_id,
                name: row.board_name,
                description: row.board_description,
                authorized_only: row.board_authorized_only,
            })
            .collect::<Vec<Self>>();

        Ok(boards)
    }

    pub async fn get_by_id(pool: &PgPool, board_id: i32) -> Result<Self, sqlx::Error> {
        let board = query_as::<_, Self>(
            //language=PostgreSQL
            r#"
        SELECT 
            board_id, 
            board_name, 
            board_description, 
            board_authorized_only
        FROM 
            board
        WHERE
            board_id = $1
        "#,
        )
        .bind(board_id)
        .fetch_one(pool)
        .await?;
        Ok(board)
    }

    pub async fn get_all_board_info(
        pool: &PgPool,
    ) -> Result<Vec<BoardEntityWithThreadInfo>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
            SELECT b.board_id,
                   b.board_name,
                   b.board_description,
                   b.board_authorized_only,
                   COUNT(t.thread_id)                                                                            AS total_threads,
                   MAX(GREATEST(t.thread_created_at,
                                COALESCE(max_reply.reply_created_at, t.thread_created_at)))                      AS most_recent_post_time,
                   MAX(t.thread_title)                                                                           AS most_recent_post_title
            FROM board b
                     LEFT JOIN
                 thread t ON b.board_id = t.thread_board_id
                     LEFT JOIN (SELECT r.reply_post_id,
                                       MAX(r.reply_created_at) AS reply_created_at
                                FROM reply r
                                GROUP BY r.reply_post_id) max_reply ON t.thread_id = max_reply.reply_post_id
            GROUP BY b.board_id, b.board_name, b.board_description, b.board_authorized_only;
        "#,
        )
            .fetch_all(pool)
            .await?;

        let boards: Vec<BoardEntityWithThreadInfo> = result
            .into_iter()
            .map(|row| BoardEntityWithThreadInfo {
                id: row.board_id,
                name: row.board_name,
                description: row.board_description,
                authorized_only: row.board_authorized_only,
                total_threads: row.total_threads.unwrap_or_default(),
                most_recent_post_time: row.most_recent_post_time.unwrap_or_default(),
                most_recent_post_title: row.most_recent_post_title.unwrap_or_default(),
            })
            .collect::<Vec<BoardEntityWithThreadInfo>>();

        Ok(boards)
    }

    pub async fn get_board_info_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<BoardEntityWithThreadInfo, sqlx::Error> {
        let board = query_as::<_, BoardEntityWithThreadInfo>(
            //language=PostgreSQL
            r#"
            SELECT b.board_id                                                                                    AS id,
                   b.board_name                                                                                  AS name,
                   b.board_description                                                                           AS description,
                   b.board_authorized_only                                                                       AS authorized_only,
                   COUNT(t.thread_id)                                                                            AS total_threads,
                   MAX(GREATEST(t.thread_created_at,
                                COALESCE(max_reply.reply_created_at, t.thread_created_at)))                      AS most_recent_post_time,
                   MAX(t.thread_title)                                                                           AS most_recent_post_title
            FROM board b
                     LEFT JOIN
                 thread t ON b.board_id = t.thread_board_id
                     LEFT JOIN (SELECT r.reply_post_id,
                                       MAX(r.reply_created_at) AS reply_created_at
                                FROM reply r
                                GROUP BY r.reply_post_id) max_reply ON t.thread_id = max_reply.reply_post_id
            WHERE b.board_id = $1
            GROUP BY b.board_id, b.board_name, b.board_description, b.board_authorized_only;
        "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(board)
    }
}
