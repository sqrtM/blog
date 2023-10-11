pub mod post;
pub mod user;

use crate::models::user::user_error::{InvalidInput, UserError};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Clone)]
pub struct AddResponse<T: Serialize + PartialEq> {
    pub(crate) status: StatusCode,
    pub(crate) content: Json<T>,
}

#[derive(Clone)]
pub struct FailResponse<T: Serialize + PartialEq + Error> {
    pub(crate) status: StatusCode,
    pub(crate) content: Json<T>,
}

impl<T: Serialize + PartialEq> IntoResponse for AddResponse<T> {
    fn into_response(self) -> Response {
        (self.status, self.content).into_response()
    }
}

impl<T: Serialize + PartialEq + Error> IntoResponse for FailResponse<T> {
    fn into_response(self) -> Response {
        (self.status, self.content.0.get_message()).into_response()
    }
}

pub trait Error {
    fn get_message(&self) -> String;
}

impl Error for UserError {
    fn get_message(&self) -> String {
        match self {
            UserError::UsernameTaken => String::from("Username Taken"),
            UserError::EmailTaken => String::from("Email Taken"),
            UserError::PasswordInvalid(e) => match e {
                InvalidInput::TooShort => {
                    String::from("Password must be longer than eight characters.")
                }
                InvalidInput::TooLong => {
                    String::from("Password must be shorter than 60 characters.")
                }
            },
            UserError::EmailInvalid(e) => match e {
                InvalidInput::TooLong => String::from("Email must be shorter than 60 characters."),
                _ => String::from("Error with email"),
            },
            UserError::UsernameInvalid(e) => match e {
                InvalidInput::TooLong => {
                    String::from("Username must be shorter than 60 characters.")
                }
                _ => String::from("Error with username"),
            },
            UserError::Unknown => String::from("Unknown input error."),
        }
    }
}
