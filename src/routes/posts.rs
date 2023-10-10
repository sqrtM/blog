use axum::routing::{get, post};
use axum::Router;

use crate::controllers::posts_controller;
use crate::AppState;

//#[axum::debug_handler]
pub fn posts_routes() -> Router<AppState> {
    Router::new().route("/", post(posts_controller::add_post))
}
