use axum::routing::post;
use axum::Router;

use crate::controllers::posts_controller;
use crate::AppState;

pub fn posts_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        post(posts_controller::add_post).get(posts_controller::get_posts_with_replies),
    )
}
