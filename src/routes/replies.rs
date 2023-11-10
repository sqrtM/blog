use axum::routing::post;
use axum::Router;

use crate::controllers::replies_controller;
use crate::AppState;

pub fn replies_routes() -> Router<AppState> {
    Router::new().route("/:post_id", post(replies_controller::get_replies_from_post))
}
