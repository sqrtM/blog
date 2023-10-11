pub mod post;
pub mod user;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Clone)]
pub struct AddResponse<T: Serialize> {
    pub(crate) status: StatusCode,
    pub(crate) message: Json<T>,
}

impl<T: Serialize> IntoResponse for AddResponse<T> {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}
