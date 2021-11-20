use crate::board::Board;
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

fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (PieceType, usize, usize)) {
    [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)].map(|transform| {
        if let Some(m) = regular_move_if_legal(
            (knight.1, knight.2),
            transform) {
            available_moves.push(m)
        }
    });
}

fn generate_pawn_moves(is_first_player_turn: bool, available_moves: &mut Vec<Move>, pawn: (PieceType, usize, usize)) {
    let move_once_row = if is_first_player_turn { pawn.2 + 1 } else { pawn.2 - 1 };
    let move_twice_row = if is_first_player_turn { pawn.2 + 2 } else { pawn.2 - 2 };
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_once_row) });
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_twice_row) });
}

fn regular_move_if_legal(starting_position: (usize, usize), transformation: (isize, isize)) -> Option<Move> {

    Some(Move::RegularMove {
        0: starting_position,
        1: (
            ((starting_position.0 as isize) + transformation.0) as usize,
            ((starting_position.1 as isize) + transformation.1) as usize
        )
    })
}