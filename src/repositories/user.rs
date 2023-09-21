use sqlx::{Error, Pool, Postgres, query_as};
use crate::models::user::User;

use crate::entities::add_user_request::AddUserRequest;

pub async fn add(pool: &Pool<Postgres>, request: AddUserRequest) -> Result<User, Error> {
    query_as!(
        User,
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
