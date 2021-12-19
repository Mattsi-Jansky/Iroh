use crate::moves::Move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank), game_state: &GameState) {
    [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)].map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform,
            game_state) {
            available_moves.push(m)
        }
    });
}

pub fn generate_king_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank), game_state: &GameState) {
    [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)].map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform,
            game_state) {
            available_moves.push(m)
        }
    });
}

fn generate_static_move_if_legal(piece: (PieceType, File, Rank), transformation: (isize, isize), game_state: &GameState) -> Option<Move> {
    let target_file = piece.1.transform(transformation.0);
    let target_rank = piece.2.transform(transformation.1);

    if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
        if let Some(target_piece) = game_state.board[(target_file, target_rank)] {
            if target_piece.is_owned_by_first_player != game_state.is_first_player_turn {
                Some(Move::AttackMove {
                    0: (piece.1, piece.2),
                    1: (target_file, target_rank),
                    2: piece.0
                })
            }
            else {None}
        } else {
            Some(Move::RegularMove {
                0: (piece.1, piece.2),
                1: (target_file, target_rank),
                2: piece.0
            })
        }
    } else {None}
}
