use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use sqlx::{query_as, Pool, Postgres};

use crate::models::user::change_password_request::ChangePasswordRequest;
use crate::models::user::user_error::InvalidInput::{TooLong, TooShort};
use crate::models::user::user_error::UserError::{PasswordInvalid, UsernameInvalid};
use crate::models::user::user_error::{InvalidInput, UserError};
use crate::models::{AddResponse, FailResponse};

#[derive(Deserialize, Clone)]
pub struct AddUserRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

struct UserRecoveryKey {
    key: Option<String>,
}

impl AddUserRequest {
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

    pub fn is_valid(&self) -> Result<(), UserError> {
        match self.validate_password() {
            Ok(_) => {}
            Err(err) => return Err(PasswordInvalid(err)),
        }
        match self.validate_username() {
            Ok(_) => {}
            Err(err) => return Err(UsernameInvalid(err)),
        }
        Ok(())
    }

    fn validate_password(&self) -> Result<(), InvalidInput> {
        // this may not be perfectly correct for pre-composed unicode chars, but
        // it's good enough for now.
        if self.password.chars().count() < 8 {
            return Err(TooShort);
        }
        if self.password.chars().count() > 60 {
            return Err(TooLong);
        }
        Ok(())
    }

    fn validate_username(&self) -> Result<(), InvalidInput> {
        if self.username.chars().count() > 60 {
            return Err(TooLong);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::models::user::add_user_request::AddUserRequest;
    use crate::models::user::user_error::UserError;

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

        let response_one = AddUserRequest::insert(&pool, request_one.clone()).await;
        let response_two = AddUserRequest::insert(&pool, request_two.clone()).await;

        assert!(response_one.ok().is_some());
        assert!(response_two.ok().is_some());

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

        let _ = AddUserRequest::insert(&pool, request_one.clone()).await;
        let response_two = AddUserRequest::insert(&pool, request_two.clone()).await;

        assert_eq!(
            response_two.err().unwrap().content.0,
            UserError::UsernameTaken
        );
    }
}
