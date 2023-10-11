use serde::Serialize;

#[derive(PartialEq, Debug, Serialize)]
pub enum UserError {
    UsernameTaken,
    EmailTaken,
    PasswordInvalid(InvalidInput),
    EmailInvalid(InvalidInput),
    UsernameInvalid(InvalidInput),
    Unknown,
}

#[derive(PartialEq, Debug, Serialize)]
pub enum InvalidInput {
    TooShort,
    TooLong,
    //Other,
    //NoNumerals,
    // ...
}

impl UserError {
    pub fn get_message(&self) -> String {
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
