use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub struct AddUserResponse {
    pub(crate) status: StatusCode,
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