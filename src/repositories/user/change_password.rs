use axum::http::StatusCode;
use axum::Json;
use sqlx::{query_as, Pool, Postgres};

use crate::models::user::change_password_request::ChangePasswordRequest;
use crate::models::user::user_error::UserError;
use crate::models::{AddResponse, FailResponse};

struct UserRecoveryKey {
    key: Option<String>,
}

pub async fn change_password(
    pool: &Pool<Postgres>,
    request: ChangePasswordRequest,
) -> Result<AddResponse<String>, FailResponse<UserError>> {
    match query_as!(
        UserRecoveryKey,
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
        request.new_password,
        request.recovery_key,
        request.old_password,
        request.username
    )
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
