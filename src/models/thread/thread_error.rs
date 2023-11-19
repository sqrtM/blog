use serde::Serialize;

use crate::models::Error;

#[derive(Serialize, PartialEq)]
pub struct ThreadError;

impl Error for ThreadError {
    fn get_message(&self) -> String {
        String::from("Problem submitting post")
    }
}
