use axum::routing::{get, post};
use axum::Router;

use crate::controllers::thread_controller;
use crate::AppState;

pub fn forum_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(thread_controller::add_thread))
        .route(
            "/:board_id",
            get(thread_controller::get_threads_with_replies),
        )
}
