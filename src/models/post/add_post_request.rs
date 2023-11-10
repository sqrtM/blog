use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use sqlx::{query_as, Pool, Postgres};
use uuid::Uuid;

use crate::models::post::post_entity::PostEntity;
use crate::models::post::post_error::PostError;
use crate::models::{AddResponse, FailResponse};

#[derive(Deserialize, Clone)]
pub struct AddPostRequest {
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
}

impl AddPostRequest {
    pub fn is_valid(&self) -> Result<(), PostError> {
        Ok(())
    }

    pub async fn insert(
        pool: &Pool<Postgres>,
        request: AddPostRequest,
    ) -> Result<AddResponse<PostEntity>, FailResponse<PostError>> {
        match query_as(
            // language=PostgreSQL
            "       
        INSERT INTO posts (post_title, post_content, post_author_id)
        VALUES ($1, $2, $3)
        RETURNING 
            post_id AS id,
            post_title AS title,
            post_content AS content,
            post_created_at AS created_at,
            post_updated_at AS updated_at,
            (SELECT user_id FROM users WHERE user_id = $3) AS author_id;
        ",
        )
        .bind(request.title)
        .bind(request.content)
        .bind(request.author_id)
        .fetch_one(pool)
        .await
        {
            Ok(post) => Ok(AddResponse {
                status: StatusCode::ACCEPTED,
                content: Json(post),
            }),
            Err(_err) => Err(FailResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content: Json(PostError),
            }),
        }
    }
}
