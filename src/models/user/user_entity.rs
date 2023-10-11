use chrono::Utc;
use serde::Serialize;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Serialize)]
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
