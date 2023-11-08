use axum::routing::{get, post};
use axum::Router;

use crate::controllers::user_controller;
use crate::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_controller::root))
        .route("/new", post(user_controller::add_user))
        .route(
            "/change-password",
            post(user_controller::change_user_password),
        )
}
