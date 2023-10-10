pub mod post;
pub mod user;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub struct AddResponse {
    pub(crate) status: StatusCode,
    pub(crate) message: String,
}

impl IntoResponse for AddResponse {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}
