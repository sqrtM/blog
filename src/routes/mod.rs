use axum::routing::get;
use axum::Router;

use crate::routes::users::user_routes;
use crate::AppState;

mod users;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
}

async fn root() -> &'static str {
    "root"
}
