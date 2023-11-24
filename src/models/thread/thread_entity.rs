use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool, Pool, Postgres, query, query_as};
use sqlx::types::Uuid;

use crate::models::FailResponse;
use crate::models::thread::add_thread_request::AddThreadRequest;
use crate::models::thread::thread_error::ThreadError;
use crate::views::NewThread;
use crate::views::thread_view::ThreadView;

#[derive(sqlx::FromRow, Serialize, PartialEq, Default)]
pub struct ThreadEntity {
    #[sqlx(rename = "thread_id")]
    pub id: Uuid,
    #[sqlx(rename = "thread_author_id")]
    pub author_id: Option<Uuid>,
    #[sqlx(rename = "thread_title")]
    pub title: String,
    #[sqlx(rename = "thread_content")]
    pub content: String,
    #[sqlx(rename = "thread_created_at")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "thread_updated_at")]
    pub updated_at: DateTime<Utc>,
    #[sqlx(rename = "thread_board_id")]
    pub board_id: Uuid,
}

impl ThreadEntity {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = query(
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
            .map(|row| ThreadEntity::from_row(&row).unwrap_or_default())
            .collect::<Vec<ThreadEntity>>();

        Ok(threads)
    }

    pub async fn get_by_board_id(pool: &PgPool, board_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        let result = query(
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
        )
            .bind(board_id)
            .fetch_all(pool)
            .await?;

        let threads: Vec<ThreadEntity> = result
            .into_iter()
            .map(|row| ThreadEntity::from_row(&row).unwrap_or_default())
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
            thread_id,
            thread_title,
            thread_content,
            thread_created_at,
            thread_updated_at,
            thread_board_id,
            (SELECT user_id FROM users WHERE user_id = $3) AS thread_author_id;
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
