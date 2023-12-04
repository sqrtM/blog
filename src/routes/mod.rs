use axum::routing::get;
use axum::Router;

use crate::routes::blog::blog_routes;
use crate::routes::board::board_routes;
use crate::routes::replies::replies_routes;
use crate::routes::threads::forum_routes;
use crate::routes::users::user_routes;
use crate::views::HomeTemplate;
use crate::AppState;

mod blog;
mod board;
mod replies;
mod threads;
mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/boards", board_routes())
        .nest("/users", user_routes())
        .nest("/forum", forum_routes())
        .nest("/replies", replies_routes())
        .nest("/blog", blog_routes())
}
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .nest("/forum", forum_routes())
        .nest("/replies", replies_routes())
}

async fn root() -> HomeTemplate {
    HomeTemplate
}
