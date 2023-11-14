use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::{Error, PgPool};
use tokio::task;
use uuid::Uuid;

use crate::models::post::add_post_request::AddPostRequest;
use crate::models::post::post_entity::PostEntity;
use crate::models::post::post_error::PostError;
use crate::models::reply::reply_entity::ReplyEntity;
use crate::models::{AddResponse, FailResponse};
use crate::views::post_view::PostView;
use crate::views::reply_view::ReplyView;
use crate::views::AllPostsPage;
use crate::AppState;

pub async fn add_post(
    State(state): State<AppState>,
    Json(request): Json<AddPostRequest>,
) -> Result<AddResponse<PostEntity>, FailResponse<PostError>> {
    match request.is_valid() {
        Ok(_) => AddPostRequest::insert(&state.db, request).await,
        Err(_e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(PostError),
        }),
    }
}

pub async fn get_posts_with_replies(State(state): State<AppState>) -> AllPostsPage {
    let posts_from_db: Result<Vec<PostEntity>, Error> = PostEntity::get_all(&state.db).await;

    let posts_for_render: Vec<Arc<Mutex<PostView>>> = posts_from_db
        .unwrap()
        .into_iter()
        .map(|post_entity| Arc::new(Mutex::new(PostView::from(post_entity))))
        .collect();

    async fn fetch_replies(post: Arc<Mutex<PostView>>, db: PgPool) {
        let id: Uuid;
        {
            let post_guard = post.lock().unwrap();
            id = post_guard.id.clone().parse().unwrap();
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

    AllPostsPage { posts }
}
