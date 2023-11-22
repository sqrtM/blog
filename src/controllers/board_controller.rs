use axum::extract::State;

use crate::AppState;
use crate::models::board::board_entity::BoardEntity;
use crate::views::AllBoardsPage;
use crate::views::board_view::BoardView;

pub async fn get_all_boards(State(state): State<AppState>) -> AllBoardsPage {
    let boards = BoardEntity::get_all(&state.db).await.unwrap_or_default();
    let mut info = vec![];
    for board in boards {
        info.append(
            &mut BoardEntity::get_board_info(&state.db, board.id)
                .await
                .unwrap_or_default(),
        );
    }
    let mut views: Vec<BoardView> = vec![];
    for board in info {
        views.push(BoardView::from(board));
    }
    AllBoardsPage {
        boards: views
    }
}
