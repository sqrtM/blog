use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query, query_as, PgPool, Pool, Postgres};

use crate::models::thread::add_thread_request::AddThreadRequest;
use crate::models::thread::thread_error::ThreadError;
use crate::models::FailResponse;
use crate::views::thread_view::ThreadView;
use crate::views::NewThread;

#[derive(sqlx::FromRow, Serialize, PartialEq)]
pub struct ThreadEntity {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub board_id: Uuid,
}

impl ThreadEntity {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
        SELECT 
            thread_id, 
            thread_author_id, 
            thread_title, 
            thread_content, 
            thread_created_at, 
            thread_updated_at,
            thread_board_id
        FROM 
            thread
        ORDER BY thread_created_at DESC;
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
                board_id: row.thread_board_id.unwrap(),
            })
            .collect::<Vec<ThreadEntity>>();

        Ok(threads)
    }

    pub async fn get_by_board_id(pool: &PgPool, board_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        let result = query!(
            //language=PostgreSQL
            r#"
        SELECT 
            thread_id, 
            thread_author_id, 
            thread_title, 
            thread_content, 
            thread_created_at, 
            thread_updated_at,
            thread_board_id
        FROM 
            thread
        WHERE 
            thread_board_id = $1
        ORDER BY thread_created_at DESC;
        "#,
            board_id
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
                board_id: row.thread_board_id.unwrap(),
            })
            .collect::<Vec<ThreadEntity>>();

        Ok(threads)
    }

    pub async fn insert(
        pool: &Pool<Postgres>,
        request: AddThreadRequest,
        board_id: Uuid,
    ) -> Result<NewThread, FailResponse<ThreadError>> {
        match query_as::<_, ThreadEntity>(
            // language=PostgreSQL
            "       
        INSERT INTO thread (thread_title, thread_content, thread_author_id, thread_board_id)
        VALUES ($1, $2, $3, $4)
        RETURNING 
            thread_id AS id,
            thread_title AS title,
            thread_content AS content,
            thread_created_at AS created_at,
            thread_updated_at AS updated_at,
            thread_board_id AS board_id,
            (SELECT user_id FROM users WHERE user_id = $3) AS author_id;
        ",
        )
        .bind(request.title)
        .bind(request.content)
        .bind(request.author_id)
        .bind(board_id)
        .fetch_one(pool)
        .await
        {
            Ok(thread) => Ok(NewThread {
                thread: ThreadView::from(thread),
            }),
            Err(_err) => Err(FailResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content: Json(ThreadError),
            }),
        }
    }
}
