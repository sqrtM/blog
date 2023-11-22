use axum::routing::get;
use axum::Router;

use crate::controllers::board_controller;
use crate::AppState;

pub fn board_routes() -> Router<AppState> {
    Router::new().route("/", get(board_controller::get_all_boards))
}
