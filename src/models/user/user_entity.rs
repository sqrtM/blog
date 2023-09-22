use chrono::Utc;
use sqlx::types::Uuid;

pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
    pub last_connection: chrono::DateTime<Utc>
}