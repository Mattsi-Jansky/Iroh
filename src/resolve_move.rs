use crate::board::Board;
use crate::moves::Move;

pub fn resolve_move(requested_move: Move, mut board: Board) -> Board {
    match requested_move {
        Move::PawnMove(from_column,(to_column,to_row)) => {
            if from_column == to_column {
                let from = (from_column,to_row-1);
                let to = (to_column,to_row);
                let piece = board[from];
                board[from] = None;
                board[to] = piece;
            } else { todo!("Pawn attack moves")}
        }
        _ => panic!("Unknown move")
    }

    board
}
