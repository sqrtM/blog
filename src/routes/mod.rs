use axum::routing::get;
use axum::Router;

use crate::routes::replies::replies_routes;
use crate::routes::threads::forum_routes;
use crate::routes::users::user_routes;
use crate::views::BaseTemplate;
use crate::AppState;

mod replies;
mod threads;
mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/test", get(test_handler))
        .nest("/users", user_routes())
        .nest("/forum", forum_routes())
        .nest("/replies", replies_routes())
}
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/test", get(test_handler))
        .nest("/users", user_routes())
        .nest("/forum", forum_routes())
        .nest("/replies", replies_routes())
}

async fn test_handler() -> &'static str {
    println!("Button Clicked!");
    "Clicked!"
}

async fn root() -> BaseTemplate {
    BaseTemplate
}
