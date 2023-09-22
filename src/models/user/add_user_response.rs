use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub struct AddUserResponse {
    pub(crate) status: StatusCode,
    pub(crate) message: String,
}

impl IntoResponse for AddUserResponse {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}
