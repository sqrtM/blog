use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};


pub struct AddUserResponse {
    pub(crate) status: StatusCode,
    pub(crate) message: &'static str,
}

impl IntoResponse for AddUserResponse {
    fn into_response(self) -> Response {
        (
            self.status,
            format!("{:?}", self.status.canonical_reason()),
        )
            .into_response()
    }
}