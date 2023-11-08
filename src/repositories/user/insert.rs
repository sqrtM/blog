use axum::http::StatusCode;
use axum::Json;
use sqlx::{query_as, Pool, Postgres};

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::user_error::UserError;
use crate::models::{AddResponse, FailResponse};

struct UserRecoveryKey {
    key: Option<String>,
}

pub async fn insert(
    pool: &Pool<Postgres>,
    request: AddUserRequest,
) -> Result<AddResponse<String>, FailResponse<UserError>> {
    match query_as!(
        UserRecoveryKey,
        // language=PostgreSQL
        "
        SELECT insert_user($1, $2) as key;
        ",
        request.username,
        request.password,
    )
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
        };

        let request_two = AddUserRequest {
            username: "two".to_string(),
            password: "two".to_string(),
        };

        let response_one = insert(&pool, request_one.clone()).await;
        let response_two = insert(&pool, request_two.clone()).await;

        assert_eq!(response_one.ok().is_some(), true);
        assert_eq!(response_two.ok().is_some(), true);

        Ok(())
    }

    #[sqlx::test]
    async fn insert_fail_on_duplicate_username(pool: PgPool) -> () {
        let request_one = AddUserRequest {
            username: "one".to_string(),
            password: "one".to_string(),
        };

        let request_two = AddUserRequest {
            username: "one".to_string(),
            password: "two".to_string(),
        };

        let _ = insert(&pool, request_one.clone()).await;
        let response_two = insert(&pool, request_two.clone()).await;

        assert_eq!(
            response_two.err().unwrap().content.0,
            UserError::UsernameTaken
        );
    }
}
