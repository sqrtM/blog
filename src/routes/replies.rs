use axum::routing::post;
use axum::Router;

use crate::controllers::replies_controller;
use crate::AppState;

pub fn replies_routes() -> Router<AppState> {
    Router::new().route("/", post(replies_controller::add_reply_to_post))
}
