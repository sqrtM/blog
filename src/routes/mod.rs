use axum::extract::State;
use axum::routing::get;
use axum::Router;
use sqlx::{Error, PgPool};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tokio::task;

use crate::models::post::post_entity::PostEntity;
use crate::models::reply::reply_entity::ReplyEntity;
use crate::routes::posts::posts_routes;
use crate::routes::replies::replies_routes;
use crate::routes::users::user_routes;
use crate::views::post_view::PostView;
use crate::views::reply_view::ReplyView;
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

    let posts_for_render: Vec<Arc<Mutex<PostView>>> = posts_from_db
        .unwrap()
        .into_iter()
        .map(|post_entity| Arc::new(Mutex::new(PostView::from(post_entity))))
        .collect();

    async fn fetch_replies(post: Arc<Mutex<PostView>>, db: PgPool) {
        let id;
        {
            let post_guard = post.lock().unwrap();
            id = post_guard.id;
        }

        let replies: Vec<ReplyView> = ReplyEntity::find_with_relations(&db, id)
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(ReplyView::from)
            .collect();

        let mut post_guard = post.lock().unwrap();
        post_guard.replies = replies;
    }

    let mut tasks = Vec::new();

    for post in &posts_for_render {
        let db = state.db.clone();
        let post_clone = Arc::clone(post);
        tasks.push(task::spawn(fetch_replies(post_clone, db)));
    }

    task::block_in_place(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { futures::future::join_all(tasks).await });
    });

    // Extract the PostView instances from the Mutex for the final result
    let posts: Vec<PostView> = posts_for_render
        .iter()
        .map(|post_mutex| post_mutex.lock().unwrap().clone())
        .collect();

    println!("{:?}", posts);

    AllPostsPage { posts }
}
