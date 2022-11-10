use crate::moves::{KING_STATIC_TRANSFORMS, KNIGHT_STATIC_TRANSFORMS, Move};
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::{Piece};

pub fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (Piece, File, Rank), game_state: &GameState, is_for_first_player: bool) {
    KNIGHT_STATIC_TRANSFORMS.map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform,
            game_state,
            is_for_first_player) {
            available_moves.push(m)
        }
    });
}

pub fn generate_king_moves(available_moves: &mut Vec<Move>, knight: (Piece, File, Rank), game_state: &GameState, is_for_first_player: bool) {
    KING_STATIC_TRANSFORMS.map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform,
            game_state,
            is_for_first_player) {
            available_moves.push(m)
        }
    });
}

fn generate_static_move_if_legal(piece: (Piece, File, Rank), transformation: (isize, isize), game_state: &GameState, is_for_first_player: bool) -> Option<Move> {
    let target_file = piece.1.transform(transformation.0);
    let target_rank = piece.2.transform(transformation.1);

    if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
        let tile = game_state.board[(target_file, target_rank)];
        if tile.is_occupied() {
            if tile.is_owned_by_first_player() != is_for_first_player {
                Some(Move::AttackMove (
                    (piece.1, piece.2),
                    (target_file, target_rank),
                    piece.0
                ))
            }
            else {None}
        } else {
            Some(Move::RegularMove (
                (piece.1, piece.2),
                (target_file, target_rank),
                piece.0
            ))
        }
    } else {None}
}
