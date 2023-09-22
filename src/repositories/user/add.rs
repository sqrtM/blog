use sqlx::{Error, Pool, Postgres, query_as, Row};

use crate::models::user::add_user_request::AddUserRequest;
use crate::models::user::user_entity::UserEntity;

pub async fn add(pool: &Pool<Postgres>, request: AddUserRequest) -> Result<UserEntity, Error> {
    query_as!(
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
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::models::user::add_user_request::AddUserRequest;
    use crate::repositories::user::add::add;

    #[sqlx::test]
    async fn add_test(pool: PgPool) -> sqlx::Result<()> {
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

        let response_one = add(&pool, request_one.clone()).await;
        let response_two = add(&pool, request_two.clone()).await;

        assert_eq!(response_one.unwrap().username, "one".to_string());
        assert_eq!(response_two.unwrap().username, "two".to_string());

        Ok(())
    }
}
