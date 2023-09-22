use sqlx::{query_as, Pool, Postgres};

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::user_entity::UserEntity;
use crate::models::user::user_error::UserError;

pub async fn insert(
    pool: &Pool<Postgres>,
    request: AddUserRequest,
) -> Result<UserEntity, UserError> {
    match query_as!(
        UserEntity,
        // language=PostgreSQL
        "
        INSERT INTO
            users
            (username, password, email)
        SELECT
            $1, crypt($2, gen_salt('bf')), crypt($3, gen_salt('bf'))
        WHERE NOT EXISTS
            (SELECT 1 FROM users WHERE email = crypt($3, email))
        RETURNING
            id, username, password, email, created_at, last_connection
        ",
        request.username,
        request.password,
        request.email
    )
    .fetch_one(pool)
    .await
    {
        Ok(user) => Ok(user),
        Err(err) => {
            match err.as_database_error() {
                None => {
                    // No error detected, but the insert failed, meaning...
                    Err(UserError::EmailTaken)
                }
                Some(db_error) => {
                    match db_error
                        .code()
                        .unwrap()
                        .into_owned()
                        .parse::<u32>()
                        .expect("No Code????")
                    {
                        23505 => Err(UserError::UsernameTaken),
                        _ => Err(UserError::Unknown),
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

        assert_eq!(response_one.unwrap().username, "one".to_string());
        assert_eq!(response_two.unwrap().username, "two".to_string());

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

        assert_eq!(response_two.err().expect("???"), UserError::UsernameTaken);
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

        assert_eq!(response_two.err().expect("???"), UserError::EmailTaken);
    }
}
