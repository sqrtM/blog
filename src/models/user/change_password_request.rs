use crate::models::user::user_error::InvalidInput;
use crate::models::user::user_error::InvalidInput::{NotEqual, TooLong, TooShort};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ChangePasswordRequest {
    pub(crate) recovery_key: String,
    pub(crate) old_password: String,
    pub(crate) new_password_check: String,
    pub(crate) new_password: String,
    pub(crate) username: String,
}

impl ChangePasswordRequest {
    pub fn is_valid(&self) -> Result<(), ()> {
        match self.validate_new_password() {
            Ok(_) => {}
            Err(_) => return Err(()),
        }
        match self.validate_passwords_are_same() {
            Ok(_) => {}
            Err(_) => return Err(()),
        }
        Ok(())
    }

    fn validate_new_password(&self) -> Result<(), InvalidInput> {
        // this may not be perfectly correct for pre-composed unicode chars, but
        // it's good enough for now.
        if self.new_password.chars().count() < 8 {
            return Err(TooShort);
        }
        if self.new_password.chars().count() > 60 {
            return Err(TooLong);
        }
        Ok(())
    }

    fn validate_passwords_are_same(&self) -> Result<(), InvalidInput> {
        if self.new_password.ne(&self.new_password_check) {
            return Err(NotEqual);
        }
        Ok(())
    }
}
