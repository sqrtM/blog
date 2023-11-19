use axum::routing::get;
use axum::Router;

use crate::controllers::replies_controller;
use crate::AppState;

pub fn replies_routes() -> Router<AppState> {
    Router::new().route(
        "/:thread_id",
        get(replies_controller::get_replies_from_thread).post(replies_controller::add_reply),
    )
}
