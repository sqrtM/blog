use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::models::user::user_error::{InvalidInput, UserError};

pub mod reply;
pub mod user;
pub mod thread;
pub mod board;
pub mod blog;

#[derive(Clone)]
pub struct GetResponse<T: Serialize + PartialEq> {
    pub(crate) status: StatusCode,
    pub(crate) content: Json<T>,
}
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

impl<T: Serialize + PartialEq> IntoResponse for GetResponse<T> {
    fn into_response(self) -> Response {
        (self.status, self.content).into_response()
    }
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
                InvalidInput::NotEqual => String::from("Passwords are not the same!"),
            },
            UserError::UsernameInvalid(e) => match e {
                InvalidInput::TooLong => {
                    String::from("Username must be shorter than 60 characters.")
                }
                InvalidInput::TooShort => String::from("Username too short!"),
                InvalidInput::NotEqual => String::from("This should never happen : 0x01"),
            },
            UserError::RecoveryKeyInvalid => String::from("Recovery key is not correct."),
            UserError::Unknown => String::from("Unknown Error."),
        }
    }
}
