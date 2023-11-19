use serde::Deserialize;
use uuid::Uuid;

use crate::models::thread::thread_error::ThreadError;

#[derive(Deserialize, Clone)]
pub struct AddReplyToPostRequest {
    pub author_id: Option<Uuid>,
    pub content: String,
    pub post_id: Uuid,
}

impl AddReplyToPostRequest {
    pub fn is_valid(&self) -> Result<(), ThreadError> {
        Ok(())
    }
}
