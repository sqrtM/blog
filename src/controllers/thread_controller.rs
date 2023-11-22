use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Form, Json};
use sqlx::Error;
use tokio::task;
use uuid::Uuid;

use crate::models::board::board_entity::BoardEntity;
use crate::models::thread::add_thread_request::AddThreadRequest;
use crate::models::thread::thread_entity::ThreadEntity;
use crate::models::thread::thread_error::ThreadError;
use crate::models::FailResponse;
use crate::services::fetch;
use crate::views::board_view::BoardView;
use crate::views::thread_view::ThreadView;
use crate::views::{NewThread, ThreadsPage};
use crate::AppState;

pub async fn add_thread(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
    Form(request): Form<AddThreadRequest>,
) -> Result<NewThread, FailResponse<ThreadError>> {
    match request.is_valid() {
        Ok(_) => ThreadEntity::insert(&state.db, request, board_id).await,
        Err(_e) => Err(FailResponse {
            status: StatusCode::BAD_REQUEST,
            content: Json(ThreadError),
        }),
    }
}

pub async fn get_threads_with_replies(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
) -> ThreadsPage {
    let threads_from_db: Result<Vec<ThreadEntity>, Error> =
        ThreadEntity::get_by_board_id(&state.db, board_id).await;

    let threads_for_render: Vec<Arc<Mutex<ThreadView>>> = threads_from_db
        .unwrap()
        .into_iter()
        .map(|thread_entity| Arc::new(Mutex::new(ThreadView::from(thread_entity))))
        .collect();

    let mut tasks = Vec::new();

    for thread in &threads_for_render {
        let db = state.db.clone();
        let thread_clone = Arc::clone(thread);
        tasks.push(task::spawn(fetch::replies_from_thread(thread_clone, db)));
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

    let board = BoardView::from(
        BoardEntity::get_board_info_by_id(&state.db, board_id)
            .await
            .unwrap_or_else(|e| panic!("{}", e.to_string())),
    );

    ThreadsPage { board, threads }
}
