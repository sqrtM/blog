use axum::routing::get;
use axum::Router;

use crate::controllers::blog_controller;
use crate::AppState;

pub fn blog_routes() -> Router<AppState> {
    Router::new()
        .route("/:blog_id", get(blog_controller::get_post))
        .route("/", get(blog_controller::get_all))
}
