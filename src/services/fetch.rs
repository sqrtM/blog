use std::sync::{Arc, Mutex};

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::reply::reply_entity::ReplyEntity;
use crate::views::reply_view::ReplyView;
use crate::views::thread_view::ThreadView;

pub(crate) async fn replies_from_thread(thread: Arc<Mutex<ThreadView>>, db: PgPool) {
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
