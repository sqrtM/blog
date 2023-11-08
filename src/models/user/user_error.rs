use serde::Serialize;

#[derive(PartialEq, Debug, Serialize)]
pub enum UserError {
    UsernameTaken,
    EmailTaken,
    PasswordInvalid(InvalidInput),
    UsernameInvalid(InvalidInput),
    RecoveryKeyInvalid,
    Unknown,
}

#[derive(PartialEq, Debug, Serialize)]
pub enum InvalidInput {
    TooShort,
    TooLong,
    NotEqual,
    //Other,
    //NoNumerals,
    // ...
}
