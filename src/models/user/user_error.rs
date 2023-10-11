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
