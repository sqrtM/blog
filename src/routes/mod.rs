use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::routes::users::user_routes;

mod users;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
}

async fn root() -> &'static str {
    "root"
}

