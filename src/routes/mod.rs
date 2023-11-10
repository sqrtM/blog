use axum::extract::State;
use axum::routing::get;
use axum::Router;
use sqlx::Error;
use tokio::task;

use crate::models::post::post_entity::PostEntity;
use crate::models::reply::reply_entity::ReplyEntity;
use crate::routes::posts::posts_routes;
use crate::routes::replies::replies_routes;
use crate::routes::users::user_routes;
use crate::views::post_view::PostView;
use crate::views::{AllPostsPage, BaseTemplate};
use crate::AppState;

mod posts;
mod replies;
mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
        .route("/posts", get(posts))
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

async fn posts(State(state): State<AppState>) -> AllPostsPage {
    let posts_from_db: Result<Vec<PostEntity>, Error> = PostEntity::get_all(&state.db).await;

    let posts_for_render: Vec<PostView> = posts_from_db
        .unwrap()
        .into_iter()
        .map(|post_entity| post_entity.into())
        .collect();

    AllPostsPage {
        posts: posts_for_render,
    }
}
