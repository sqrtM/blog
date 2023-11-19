use serde::Deserialize;
use uuid::Uuid;

use crate::models::thread::thread_error::ThreadError;

#[derive(Deserialize, Clone, Debug)]
pub struct AddReplyToThreadRequest {
    pub author_id: Option<Uuid>,
    pub content: String,
}

impl AddReplyToThreadRequest {
    pub fn is_valid(&self) -> Result<(), ThreadError> {
        Ok(())
    }
}
