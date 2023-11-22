use axum::extract::State;

use crate::models::board::board_entity::BoardEntity;
use crate::views::board_view::BoardView;
use crate::views::AllBoardsPage;
use crate::AppState;

pub async fn get_all_boards(State(state): State<AppState>) -> AllBoardsPage {
    let boards = BoardEntity::get_all_board_info(&state.db)
        .await
        .unwrap_or_default();
    let mut views: Vec<BoardView> = vec![];
    for board in boards {
        views.push(BoardView::from(board));
    }
    AllBoardsPage { boards: views }
}
