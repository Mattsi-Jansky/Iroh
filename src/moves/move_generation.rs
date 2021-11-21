use crate::board::Board;
use crate::coordinates::{File, Rank};
use crate::moves::Move;
use crate::piece::PieceType;

pub fn generate_moves(is_first_player_turn: bool, board: &Board) -> Vec<Move> {
    let mut available_moves = vec![];

    let pieces = board.get_all_pieces_belonging_to_player(is_first_player_turn);
    for piece in pieces {
        match piece.0 {
            PieceType::Pawn => generate_pawn_moves(is_first_player_turn,&mut available_moves,piece),
            PieceType::Knight => generate_knight_moves(&mut available_moves, piece),
            _ => {}
        }
    }

    available_moves
}

fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank)) {
    [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)].map(|transform| {
        if let Some(m) = regular_move_if_legal(
            knight,
            transform) {
            available_moves.push(m)
        }
    });
}

fn generate_pawn_moves(is_first_player_turn: bool, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank)) {
    let move_once_rank = if is_first_player_turn { pawn.2 + 1 } else { pawn.2 - 1 };
    let move_twice_rank = if is_first_player_turn { pawn.2 + 2 } else { pawn.2 - 2 };
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_once_rank) });
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_twice_rank) });
}

fn regular_move_if_legal(piece: (PieceType, File, Rank), transformation: (isize, isize)) -> Option<Move> {
    let target_file = piece.1.transform(transformation.0);
    let target_rank = piece.2.transform(transformation.1);

    if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
        Some(Move::RegularMove {
            0: (piece.1, piece.2),
            1: (target_file,target_rank),
            2: piece.0
        })
    } else {None}
}