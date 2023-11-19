use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use sqlx::{query_as, Pool, Postgres};
use uuid::Uuid;

use crate::models::thread::thread_entity::ThreadEntity;
use crate::models::thread::thread_error::ThreadError;
use crate::models::{AddResponse, FailResponse};

#[derive(Deserialize, Clone, Debug)]
pub struct AddThreadRequest {
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
}

impl AddThreadRequest {
    pub fn is_valid(&self) -> Result<(), ThreadError> {
        Ok(())
    }

    pub async fn insert(
        pool: &Pool<Postgres>,
        request: AddThreadRequest,
    ) -> Result<AddResponse<ThreadEntity>, FailResponse<ThreadError>> {
        match query_as(
            // language=PostgreSQL
            "       
        INSERT INTO thread (thread_title, thread_content, thread_author_id)
        VALUES ($1, $2, $3)
        RETURNING 
            thread_id AS id,
            thread_title AS title,
            thread_content AS content,
            thread_created_at AS created_at,
            thread_updated_at AS updated_at,
            (SELECT user_id FROM users WHERE user_id = $3) AS author_id;
        ",
        )
        .bind(request.title)
        .bind(request.content)
        .bind(request.author_id)
        .fetch_one(pool)
        .await
        {
            Ok(thread) => Ok(AddResponse {
                status: StatusCode::ACCEPTED,
                content: Json(thread),
            }),
            Err(_err) => Err(FailResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content: Json(ThreadError),
            }),
        }
    }
}
