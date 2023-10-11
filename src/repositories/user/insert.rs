use axum::http::StatusCode;
use axum::Json;
use sqlx::{query_as, Pool, Postgres};

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::user_entity::UserEntity;
use crate::models::user::user_error::UserError;
use crate::models::{AddResponse, FailResponse};

pub async fn insert(
    pool: &Pool<Postgres>,
    request: AddUserRequest,
) -> Result<AddResponse<UserEntity>, FailResponse<UserError>> {
    match query_as!(
        UserEntity,
        // language=PostgreSQL
        "
        INSERT INTO
            users
            (user_username, user_password, user_email)
        SELECT
            $1, crypt($2, gen_salt('bf')), crypt($3, gen_salt('bf'))
        WHERE NOT EXISTS (
            SELECT 
                1 
            FROM 
                users 
            WHERE 
                user_email = crypt($3, user_email)
            )
        RETURNING
            user_id AS id, 
            user_username AS username, 
            user_password AS password, 
            user_email AS email, 
            user_created_at AS created_at, 
            user_last_connection AS last_connection
        ",
        request.username,
        request.password,
        request.email
    )
    .fetch_one(pool)
    .await
    {
        Ok(user) => Ok(AddResponse {
            status: StatusCode::ACCEPTED,
            content: Json(user),
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

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::models::user::add_user_request::AddUserRequest;
    use crate::models::user::user_error::UserError;
    use crate::repositories::user::insert::insert;

    #[sqlx::test]
    async fn insert_successfully(pool: PgPool) -> sqlx::Result<()> {
        let request_one = AddUserRequest {
            username: "one".to_string(),
            password: "one".to_string(),
            email: "one@one.com".to_string(),
        };

        let request_two = AddUserRequest {
            username: "two".to_string(),
            password: "two".to_string(),
            email: "two@two.com".to_string(),
        };

        let response_one = insert(&pool, request_one.clone()).await;
        let response_two = insert(&pool, request_two.clone()).await;

        assert_eq!(
            response_one.ok().unwrap().content.0.username,
            "one".to_string()
        );
        assert_eq!(
            response_two.ok().unwrap().content.0.username,
            "two".to_string()
        );

        Ok(())
    }

    #[sqlx::test]
    async fn insert_fail_on_duplicate_username(pool: PgPool) -> () {
        let request_one = AddUserRequest {
            username: "one".to_string(),
            password: "one".to_string(),
            email: "one@one.com".to_string(),
        };

        let request_two = AddUserRequest {
            username: "one".to_string(),
            password: "two".to_string(),
            email: "two@two.com".to_string(),
        };

        let _ = insert(&pool, request_one.clone()).await;
        let response_two = insert(&pool, request_two.clone()).await;

        assert_eq!(
            response_two.err().unwrap().content.0,
            UserError::UsernameTaken
        );
    }

    #[sqlx::test]
    async fn insert_fail_on_duplicate_email(pool: PgPool) -> () {
        let request_one = AddUserRequest {
            username: "one".to_string(),
            password: "one".to_string(),
            email: "one@one.com".to_string(),
        };

        let request_two = AddUserRequest {
            username: "two".to_string(),
            password: "two".to_string(),
            email: "one@one.com".to_string(),
        };

        let _ = insert(&pool, request_one.clone()).await;
        let response_two = insert(&pool, request_two.clone()).await;

        assert_eq!(response_two.err().unwrap().content.0, UserError::EmailTaken);
    }
}
