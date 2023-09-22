use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AddUserRequest {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) email: String,
}

impl AddUserRequest {
    pub fn is_valid(&self) -> bool {
        self.validate_password()
    }

    fn validate_password(&self) -> bool {
        self.password.len() >= 8
    }
}