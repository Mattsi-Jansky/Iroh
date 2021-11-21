use crate::board::Board;
use crate::moves::Move;

pub fn resolve_move(requested_move: Move, mut board: Board) -> Board {
    match requested_move {
        Move::PawnMove(from_rank, (to_file, to_rank)) => {
            if from_rank == to_file {
                let from = (from_rank, to_rank -1);
                let to = (to_file, to_rank);
                let piece = board[from];
                board[from] = None;
                board[to] = piece;
            } else { todo!("Pawn attack moves")}
        }
        Move::RegularMove((from_file, from_rank), (to_file, to_rank), _) => {
            let from = (from_file,from_rank);
            let to = (to_file,to_rank);
            let piece = board[from];
            board[from] = None;
            board[to] = piece;
        }
    }

    board
}
