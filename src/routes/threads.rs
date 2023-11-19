use axum::routing::post;
use axum::Router;

use crate::controllers::thread_controller;
use crate::AppState;

pub fn forum_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        post(thread_controller::add_thread).get(thread_controller::get_threads_with_replies),
    )
}
