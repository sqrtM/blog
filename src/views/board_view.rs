use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::board::board_entity::BoardEntityWithThreadInfo;

#[derive(Serialize, Clone, Debug)]
pub struct BoardView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authorized_only: bool,
    pub total_threads: i64,
    pub most_recent_post_time: DateTime<Utc>,
    pub most_recent_post_title: String,
}

impl From<BoardEntityWithThreadInfo> for BoardView {
    fn from(board_entity: BoardEntityWithThreadInfo) -> Self {
        Self {
            id: board_entity.id.hyphenated().to_string(),
            name: board_entity.name,
            description: board_entity.description,
            authorized_only: board_entity.authorized_only,
            total_threads: board_entity.total_threads,
            most_recent_post_time: board_entity.most_recent_post_time,
            most_recent_post_title: board_entity.most_recent_post_title,
        }
    }
}
