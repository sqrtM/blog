use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use serde::Serialize;
use sqlx::{Pool, Postgres, query_as};
use sqlx::types::Uuid;

use crate::models::{AddResponse, FailResponse};
use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::change_password_request::ChangePasswordRequest;
use crate::models::user::user_error::UserError;

#[derive(sqlx::FromRow, Serialize, PartialEq)]
pub struct UserEntity {
    #[sqlx(rename = "user_id")]
    pub id: Uuid,
    #[sqlx(rename = "user_username")]
    pub username: String,
    #[sqlx(rename = "user_password")]
    pub password: String,
    #[sqlx(rename = "user_email")]
    pub email: String,
    #[sqlx(rename = "user_created_at")]
    pub created_at: chrono::DateTime<Utc>,
    #[sqlx(rename = "user_last_connection")]
    pub last_connection: chrono::DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct UserRecoveryKey {
    key: Option<String>,
}

impl UserEntity {
    pub async fn insert(
        pool: &Pool<Postgres>,
        request: AddUserRequest,
    ) -> Result<AddResponse<String>, FailResponse<UserError>> {
        match query_as::<_, UserRecoveryKey>(
            // language=PostgreSQL
            "
        SELECT insert_user($1, $2) as key;
        ",
        )
            .bind(request.username)
            .bind(request.password)
            .fetch_one(pool)
            .await
        {
            Ok(key) => Ok(AddResponse {
                status: StatusCode::ACCEPTED,
                content: Json(key.key.unwrap()),
            }),
            Err(err) => {
                match err.as_database_error() {
                    None => {
                        // No error detected, but the insert failed, meaning...
                        Err(FailResponse {
                            status: StatusCode::BAD_REQUEST,
                            content: Json(UserError::EmailTaken),
                        })
                    }
                    Some(db_error) => {
                        match db_error
                            .code()
                            .unwrap()
                            .into_owned()
                            .parse::<u32>()
                            .expect("No Code????")
                        {
                            23505 => Err(FailResponse {
                                status: StatusCode::BAD_REQUEST,
                                content: Json(UserError::UsernameTaken),
                            }),
                            _ => Err(FailResponse {
                                status: StatusCode::BAD_REQUEST,
                                content: Json(UserError::Unknown),
                            }),
                        }
                    }
                }
            }
        }
    }

    pub async fn change_password(
        pool: &Pool<Postgres>,
        request: ChangePasswordRequest,
    ) -> Result<AddResponse<String>, FailResponse<UserError>> {
        match query_as::<_, UserRecoveryKey>(
            // language=PostgreSQL
            "
        WITH new_key AS (
            SELECT encode(digest(gen_random_bytes(16), 'sha1'), 'hex') AS unhashed_key
        )
        UPDATE users
        SET user_password = crypt($1, gen_salt('bf')), 
            user_recovery_key = crypt((SELECT unhashed_key FROM new_key), gen_salt('bf'))
        WHERE user_recovery_key = crypt($2, user_recovery_key)  
          AND user_password = crypt($3, user_password) 
          AND user_username = $4
        RETURNING (SELECT unhashed_key FROM new_key) AS key;
        ",
        )
            .bind(request.new_password)
            .bind(request.recovery_key)
            .bind(request.old_password)
            .bind(request.username)
            .fetch_one(pool)
            .await
        {
            Ok(key) => Ok(AddResponse {
                status: StatusCode::ACCEPTED,
                content: Json(key.key.unwrap()),
            }),
            Err(_) => Err(FailResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                content: Json(UserError::Unknown),
            }),
        }
    }
}

#[cfg(test)]
mod db_tests {
    use sqlx::PgPool;

    use crate::models::user::add_user_request::AddUserRequest;
    use crate::models::user::user_entity::UserEntity;
    use crate::models::user::user_error::UserError;

    #[sqlx::test]
    #[ignore]
    async fn insert_successfully(pool: PgPool) -> sqlx::Result<()> {
        let request_one = AddUserRequest {
            username: "one".to_string(),
            password: "one".to_string(),
        };

        let request_two = AddUserRequest {
            username: "two".to_string(),
            password: "two".to_string(),
        };

        let response_one = UserEntity::insert(&pool, request_one.clone()).await;
        let response_two = UserEntity::insert(&pool, request_two.clone()).await;

        assert!(response_one.ok().is_some());
        assert!(response_two.ok().is_some());

        Ok(())
    }

    #[sqlx::test]
    #[ignore]
    async fn insert_fail_on_duplicate_username(pool: PgPool) -> () {
        let request_one = AddUserRequest {
            username: "one".to_string(),
            password: "one".to_string(),
        };

        let request_two = AddUserRequest {
            username: "one".to_string(),
            password: "two".to_string(),
        };

        let _ = UserEntity::insert(&pool, request_one.clone()).await;
        let response_two = UserEntity::insert(&pool, request_two.clone()).await;

        assert_eq!(
            response_two.err().unwrap().content.0,
            UserError::UsernameTaken
        );
    }
}
