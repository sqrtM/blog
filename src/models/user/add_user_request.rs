use serde::Deserialize;

use crate::models::user::user_error::{InvalidInput, UserError};
use crate::models::user::user_error::InvalidInput::{TooLong, TooShort};
use crate::models::user::user_error::UserError::{PasswordInvalid, UsernameInvalid};

#[derive(Deserialize, Clone)]
pub struct AddUserRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

struct UserRecoveryKey {
    key: Option<String>,
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
}
