use crate::models::post::post_error::PostError;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct AddPostRequest {
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
}

impl AddPostRequest {
    pub fn is_valid(&self) -> Result<(), PostError> {
        Ok(())
    }
}
