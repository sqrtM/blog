use serde::Deserialize;

use crate::models::user::user_error::InvalidInput::{TooLong, TooShort};
use crate::models::user::user_error::UserError::{EmailInvalid, PasswordInvalid, UsernameInvalid};
use crate::models::user::user_error::{InvalidInput, UserError};

#[derive(Deserialize, Clone)]
pub struct AddUserRequest {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) email: String,
}

impl AddUserRequest {
    pub fn is_valid(&self) -> Result<(), UserError> {
        match self.validate_password() {
            Ok(_) => {}
            Err(err) => return Err(PasswordInvalid(err)),
        }
        match self.validate_username() {
            Ok(_) => {}
            Err(err) => return Err(UsernameInvalid(err)),
        }
        match self.validate_email() {
            Ok(_) => {}
            Err(err) => return Err(EmailInvalid(err)),
        }
        Ok(())
    }

    fn validate_password(&self) -> Result<(), InvalidInput> {
        // this may not be perfectly correct for pre-composed unicode chars, but
        // it's good enough for now.
        if self.password.chars().count() < 8 {
            return Err(TooShort);
        }
        if self.password.chars().count() > 60 {
            return Err(TooLong);
        }
        Ok(())
    }

    fn validate_username(&self) -> Result<(), InvalidInput> {
        if self.username.chars().count() > 60 {
            return Err(TooLong);
        }
        Ok(())
    }

    fn validate_email(&self) -> Result<(), InvalidInput> {
        if self.email.chars().count() > 60 {
            return Err(TooLong);
        }
        Ok(())
    }
}
