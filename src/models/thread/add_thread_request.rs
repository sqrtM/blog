use serde::Deserialize;
use uuid::Uuid;

use crate::models::thread::thread_error::ThreadError;

#[derive(Deserialize, Clone, Debug)]
pub struct AddThreadRequest {
    pub author_id: Option<Uuid>,
    pub title: String,
    pub content: String,
}

impl AddThreadRequest {
    pub fn is_valid(&self) -> Result<(), ThreadError> {
        Ok(())
    }
}
