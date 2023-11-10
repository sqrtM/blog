use crate::models::post::post_error::PostError;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct AddReplyToPostRequest {
    pub author_id: Option<Uuid>,
    pub content: String,
    pub post_id: Uuid
}

impl AddReplyToPostRequest {
    pub fn is_valid(&self) -> Result<(), PostError> {
        Ok(())
    }
}
