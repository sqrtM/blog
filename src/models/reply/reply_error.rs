use serde::Serialize;

use crate::models::Error;

#[derive(Serialize, PartialEq)]
pub struct ReplyError;

impl Error for ReplyError {
    fn get_message(&self) -> String {
        String::from("Problem submitting reply")
    }
}
