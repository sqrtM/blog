use chrono::Utc;
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
    pub last_connection: chrono::DateTime<Utc>
}