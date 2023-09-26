use axum::routing::get;
use axum::Router;

use crate::controllers::user_controller;
use crate::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        get(user_controller::root).post(user_controller::add_user),
    )
}
