use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::{Form, Json};
use sqlx::{Error, PgPool};
use tokio::task;
use uuid::Uuid;

use crate::models::reply::reply_entity::ReplyEntity;
use crate::models::thread::add_thread_request::AddThreadRequest;
use crate::models::thread::thread_entity::ThreadEntity;
use crate::models::thread::thread_error::ThreadError;
use crate::models::FailResponse;
use crate::views::reply_view::ReplyView;
use crate::views::thread_view::ThreadView;
use crate::views::{AllThreadsPage, NewThread};
use crate::AppState;

pub async fn add_thread(
    State(state): State<AppState>,
    Form(request): Form<AddThreadRequest>,
) -> Result<NewThread, FailResponse<ThreadError>> {
    match request.is_valid() {
        Ok(_) => ThreadEntity::insert(&state.db, request).await,
        Err(_e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(ThreadError),
        }),
    }
}

pub async fn get_threads_with_replies(State(state): State<AppState>) -> AllThreadsPage {
    let threads_from_db: Result<Vec<ThreadEntity>, Error> = ThreadEntity::get_all(&state.db).await;

    let threads_for_render: Vec<Arc<Mutex<ThreadView>>> = threads_from_db
        .unwrap()
        .into_iter()
        .map(|thread_entity| Arc::new(Mutex::new(ThreadView::from(thread_entity))))
        .collect();

    async fn fetch_replies(thread: Arc<Mutex<ThreadView>>, db: PgPool) {
        let id: Uuid;
        {
            let thread_guard = thread.lock().unwrap();
            id = thread_guard.id.clone().parse().unwrap();
        }

        let replies: Vec<ReplyView> = ReplyEntity::find_with_relations(&db, id)
            .await
            .unwrap_or_else(|e| panic!("{:?}", e.to_string()))
            .into_iter()
            .map(ReplyView::from)
            .collect();

        let mut thread_guard = thread.lock().unwrap();
        thread_guard.replies = replies;
    }

    let mut tasks = Vec::new();

    for thread in &threads_for_render {
        let db = state.db.clone();
        let thread_clone = Arc::clone(thread);
        tasks.push(task::spawn(fetch_replies(thread_clone, db)));
    }

    task::block_in_place(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { futures::future::join_all(tasks).await });
    });

    let threads: Vec<ThreadView> = threads_for_render
        .iter()
        .map(|thread_mutex| thread_mutex.lock().unwrap().clone())
        .collect();

    AllThreadsPage { threads }
}
