use crate::moves::coordinate_transformers::{
    DIAGONAL_DYNAMIC_TRANSFORMERS, STRAIGHT_AND_DIAGONAL_TRANSFORMERS,
    STRAIGHT_DYNAMIC_TRANSFORMERS,
};
use crate::moves::Move;
use crate::state::coordinates::Coordinate;
use crate::state::tile::Tile;
use crate::state::GameState;

pub fn generate_queen_moves(
    available_moves: &mut Vec<Move>,
    queen: (Tile, Coordinate),
    game_state: &GameState,
    is_for_first_player: bool,
) {
    generate_dynamic_moves(
        game_state,
        available_moves,
        queen,
        &STRAIGHT_AND_DIAGONAL_TRANSFORMERS,
        is_for_first_player,
    );
}

pub fn generate_bishop_moves(
    available_moves: &mut Vec<Move>,
    bishop: (Tile, Coordinate),
    game_state: &GameState,
    is_for_first_player: bool,
) {
    generate_dynamic_moves(
        game_state,
        available_moves,
        bishop,
        &DIAGONAL_DYNAMIC_TRANSFORMERS,
        is_for_first_player,
    );
}

pub fn generate_rook_moves(
    available_moves: &mut Vec<Move>,
    rook: (Tile, Coordinate),
    game_state: &GameState,
    is_for_first_player: bool,
) {
    generate_dynamic_moves(
        game_state,
        available_moves,
        rook,
        &STRAIGHT_DYNAMIC_TRANSFORMERS,
        is_for_first_player,
    );
}

fn generate_dynamic_moves(
    game_state: &GameState,
    available_moves: &mut Vec<Move>,
    origin: (Tile, Coordinate),
    transformers: &[fn(Coordinate) -> Option<Coordinate>],
    is_for_first_player: bool,
) {
    for transformer in transformers {
        let mut coordinate = Some(origin.1);
        loop {
            coordinate = transformer(coordinate.unwrap());
            if let Some(coordinate) = coordinate {
                let tile = game_state.board[coordinate];
                if tile.is_occupied() {
                    if tile.is_owned_by_first_player() != is_for_first_player {
                        available_moves.push(Move::AttackMove(origin.1, coordinate, origin.0))
                    }
                    break;
                } else {
                    available_moves.push(Move::RegularMove(origin.1, coordinate, origin.0))
                }
            } else {
                break;
            }
        }
    }
}
