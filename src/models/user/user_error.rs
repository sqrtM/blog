#[derive(PartialEq, Debug)]
pub enum UserError {
    UsernameTaken,
    EmailTaken,
    PasswordInvalid(InvalidPasswordReason),
    Unknown,
}

#[derive(PartialEq, Debug)]
pub enum InvalidPasswordReason {
    LessThanEightCharacters,
    //NoNumerals,
    // ...
}

impl UserError {
    pub fn get_message(&self) -> String {
        match self {
            UserError::UsernameTaken => String::from("Username Taken"),
            UserError::EmailTaken => String::from("Email Taken"),
            UserError::PasswordInvalid(_) => String::from("Password Invalid"),
            UserError::Unknown => String::from("No idea"),
        }
    }
}
