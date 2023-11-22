use crate::controllers::board_controller;
use crate::AppState;
use axum::routing::get;
use axum::Router;

pub fn board_routes() -> Router<AppState> {
    Router::new().route("/", get(board_controller::get_all_boards))
}
