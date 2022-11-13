use crate::moves::{Move};
use crate::moves::coordinate_transformers::{KING_STATIC_TRANSFORMERS, KNIGHT_STATIC_TRANSFORMERS};
use crate::state::coordinates::Coordinate;
use crate::state::GameState;
use crate::state::tile::{Tile};

pub fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (Tile, Coordinate), game_state: &GameState, is_for_first_player: bool) {
    KNIGHT_STATIC_TRANSFORMERS.map(|transformer| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transformer,
            game_state,
            is_for_first_player) {
            available_moves.push(m)
        }
    });
}

pub fn generate_king_moves(available_moves: &mut Vec<Move>, king: (Tile, Coordinate), game_state: &GameState, is_for_first_player: bool) {
    KING_STATIC_TRANSFORMERS.map(|transformer| {
        if let Some(m) = generate_static_move_if_legal(
            king,
            transformer,
            game_state,
            is_for_first_player) {
            available_moves.push(m)
        }
    });
}

fn generate_static_move_if_legal(origin: (Tile, Coordinate), transformer: fn(Coordinate) -> Option<Coordinate>, game_state: &GameState, is_for_first_player: bool) -> Option<Move> {
    let target_coordinate = transformer(origin.1);

    if let Some(target_coordinate) = target_coordinate {
        let tile = game_state.board[target_coordinate];
        if tile.is_occupied() {
            if tile.is_owned_by_first_player() != is_for_first_player {
                Some(Move::AttackMove (
                    origin.1,
                    target_coordinate,
                    origin.0
                ))
            }
            else {None}
        } else {
            Some(Move::RegularMove (
                origin.1,
                target_coordinate,
                origin.0
            ))
        }
    } else {None}
}
