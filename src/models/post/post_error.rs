use serde::Serialize;

use crate::models::Error;

#[derive(Serialize, PartialEq)]
pub struct PostError;

impl Error for PostError {
    fn get_message(&self) -> String {
        String::from("Problem submitting post")
    }
}
