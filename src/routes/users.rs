use axum::Router;
use axum::routing::get;
use crate::AppState;
use crate::controllers::user_controller;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/",
               get(user_controller::root)
                   .post(user_controller::add_user),
        )
}