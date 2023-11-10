use axum::routing::get;
use axum::Router;

use crate::html::BaseTemplate;
use crate::routes::posts::posts_routes;
use crate::routes::replies::replies_routes;
use crate::routes::users::user_routes;
use crate::AppState;

mod posts;
mod replies;
mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .nest("/posts", posts_routes())
        .nest("/replies", replies_routes())
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .nest("/posts", posts_routes())
        .nest("/replies", replies_routes())
}

async fn root() -> BaseTemplate {
    BaseTemplate
}
