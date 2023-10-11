use crate::models::{AddResponse, FailResponse};
use axum::http::StatusCode;
use axum::Json;
use sqlx::{query_as, Pool, Postgres};

use crate::models::post::add_post_request::AddPostRequest;
use crate::models::post::post_entity::PostEntity;
use crate::models::post::post_error::PostError;

pub async fn insert(
    pool: &Pool<Postgres>,
    request: AddPostRequest,
) -> Result<AddResponse<PostEntity>, FailResponse<PostError>> {
    match query_as(
        // language=PostgreSQL
        "       
        WITH inserted_post AS (
          INSERT INTO posts (post_title, post_content, post_author_id)
          VALUES ($1, $2, $3)
          RETURNING 
              post_title, 
              post_content, 
              post_author_id, 
              post_id, 
              post_created_at, 
              post_updated_at
        )
        SELECT 
            inserted_post.post_id AS post_id, 
            inserted_post.post_title AS post_title, 
            inserted_post.post_content AS post_content, 
            inserted_post.post_created_at AS post_created_at,
            inserted_post.post_updated_at AS post_updated_at,
            
            u.user_id AS user_id,
            u.user_username AS user_username,
            u.user_password AS user_password,
            u.user_email AS user_email,
            u.user_created_at AS user_created_at,
            u.user_last_connection AS user_last_connection
        
        FROM inserted_post
        JOIN users u ON inserted_post.post_author_id = u.user_id;
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
