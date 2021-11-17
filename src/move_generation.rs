use crate::board::Board;
use crate::moves::Move;
use crate::piece::ChessPieceType;

pub fn generate_moves(is_first_player_turn: bool, board: &Board) -> Vec<Move> {
    let mut available_moves = vec![];

    let pawns = board.get_all(ChessPieceType::Pawn, is_first_player_turn);
    for pawn in pawns {
        let move_once_row = if is_first_player_turn {pawn.2 + 1} else {pawn.2 - 1};
        let move_twice_row = if is_first_player_turn {pawn.2 + 2} else {pawn.2 - 2};
        available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_once_row) });
        available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_twice_row) });
    }

    available_moves
}
