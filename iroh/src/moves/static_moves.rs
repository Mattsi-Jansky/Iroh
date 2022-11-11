use crate::moves::{KING_STATIC_TRANSFORMS, KNIGHT_STATIC_TRANSFORMS, Move};
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::tile::{Tile};

pub fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (Tile, File, Rank), game_state: &GameState, is_for_first_player: bool) {
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

pub fn generate_king_moves(available_moves: &mut Vec<Move>, knight: (Tile, File, Rank), game_state: &GameState, is_for_first_player: bool) {
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

fn generate_static_move_if_legal(from_tile: (Tile, File, Rank), transformation: (isize, isize), game_state: &GameState, is_for_first_player: bool) -> Option<Move> {
    let target_file = from_tile.1.transform(transformation.0);
    let target_rank = from_tile.2.transform(transformation.1);

    if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
        let tile = game_state.board[(target_file, target_rank)];
        if tile.is_occupied() {
            if tile.is_owned_by_first_player() != is_for_first_player {
                Some(Move::AttackMove (
                    (from_tile.1, from_tile.2),
                    (target_file, target_rank),
                    from_tile.0
                ))
            }
            else {None}
        } else {
            Some(Move::RegularMove (
                (from_tile.1, from_tile.2),
                (target_file, target_rank),
                from_tile.0
            ))
        }
    } else {None}
}
