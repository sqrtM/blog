use sqlx::{Error, Pool, Postgres, query_as};
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
        VALUES
            ($1, crypt($2, gen_salt('bf')), crypt($3, gen_salt('bf')))
        RETURNING id, username, password, email, created_at, last_connection
        ",
        request.username,
        request.password,
        request.email
    )
        .fetch_one(pool)
        .await
}
