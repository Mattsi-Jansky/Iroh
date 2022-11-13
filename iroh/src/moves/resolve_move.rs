use crate::moves::Move;
use crate::state::coordinates::Coordinate;
use crate::state::GameState;
use crate::state::tile::{Tile};

pub fn resolve_move(requested_move: &Move, game_state: GameState) -> GameState {
    let is_first_player_turn = game_state.is_first_player_turn;
    resolve_move_for(requested_move, game_state, is_first_player_turn)
}

pub fn resolve_move_for(requested_move: &Move, mut game_state: GameState, is_first_player: bool) -> GameState {
    match requested_move {
        Move::PawnMove(from, to) => {
            move_piece(&mut game_state, from, to);
        }
        Move::RegularMove(from, to, _) => {
            // move_piece(&mut game_state, from_file, from_rank, to_file, to_rank);
        }
        Move::AttackMove(from, to, _) => {
            // let tile = game_state.board[(to_file, to_rank)];
            // assert!(tile.is_occupied(), "Illegal move, no target to attack");
            // if is_first_player {game_state.captured_pieces.captured_second_player(tile, game_state.turn_number);}
            // else {game_state.captured_pieces.captured_first_player(tile, game_state.turn_number);}
            //
            // move_piece(&mut game_state, from_file, from_rank, to_file, to_rank);
        }
        Move::PawnAttackMove(from, to) => {
            // let tile = game_state.board[(to_file, to_rank)];
            // assert!(tile.is_occupied(), "Illegal move, no target to attack");
            // if is_first_player {game_state.captured_pieces.captured_second_player(tile, game_state.turn_number);}
            // else {game_state.captured_pieces.captured_first_player(tile, game_state.turn_number);}
            //
            // let direction = if is_first_player {-1} else {1};
            // move_piece(&mut game_state,
            //            from,
            //            &to_rank.transform(direction)
            //                .expect("should not be possible to get an out of bounds pawn attack command"),
            //            to_file,
            //            to_rank);
        }
        Move::PawnPromotion(target, tile) => {
            // let from_rank = &Rank::new(if tile.is_owned_by_first_player() {6} else {1});
            // let to_rank = &Rank::new(if tile.is_owned_by_first_player() {7} else {0});
            // game_state.board[(target, from_rank)] = Tile::EMPTY;
            // game_state.board[(target, to_rank)] = tile.clone()
        }
        Move::Castle(is_kingside) => {
            // match (is_first_player, *is_kingside) {
            //     (true, true) => {
            //         move_piece(&mut game_state, &File::E, &Rank::ONE,
            //                    &File::G, &Rank::ONE);
            //         move_piece(&mut game_state, &File::H, &Rank::ONE,
            //                    &File::F, &Rank::ONE);
            //     }
            //     (true, false) => {
            //         move_piece(&mut game_state, &File::E, &Rank::ONE,
            //                    &File::C, &Rank::ONE);
            //         move_piece(&mut game_state, &File::A, &Rank::ONE,
            //                    &File::D, &Rank::ONE);
            //     }
            //     (false, true) => {
            //         move_piece(&mut game_state, &File::E, &Rank::EIGHT,
            //                    &File::G, &Rank::EIGHT);
            //         move_piece(&mut game_state, &File::H, &Rank::EIGHT,
            //                    &File::F, &Rank::EIGHT);
            //     }
            //     (false, false) => {
            //         move_piece(&mut game_state, &File::E, &Rank::EIGHT,
            //                    &File::C, &Rank::EIGHT);
            //         move_piece(&mut game_state, &File::A, &Rank::EIGHT,
            //                    &File::D, &Rank::EIGHT);
            //     }
            // }
        }
    }

    game_state.next_turn();
    game_state
}

fn move_piece(game_state: &mut GameState, from: &Coordinate, to: &Coordinate) {
    let tile = game_state.board[from];
    game_state.board[from] = Tile::EMPTY;
    game_state.board[to] = tile;

    update_castling_state(game_state, from, tile)
}

fn update_castling_state(game_state: &mut GameState, from: &Coordinate, tile: Tile) {
    if tile == Tile::FIRST_ROOK
        && from == &Coordinate::H1 {
        game_state.first_player_can_castle_kingside = false;
    }
    else if tile == Tile::FIRST_ROOK
        && from == &Coordinate::A1 {
        game_state.first_player_can_castle_queenside = false;
    }
    else if tile == Tile::FIRST_KING
        && from == &Coordinate::E1 {
        game_state.first_player_can_castle_kingside = false;
        game_state.first_player_can_castle_queenside = false;
    }

    if tile == Tile::SECOND_ROOK
        && from == &Coordinate::H8 {
        game_state.second_player_can_castle_kingside = false;
    }
    else if tile == Tile::SECOND_ROOK
        && from == &Coordinate::A8 {
        game_state.second_player_can_castle_queenside = false;
    }
    else if tile == Tile::SECOND_KING
        && from == &Coordinate::E8 {
        game_state.second_player_can_castle_kingside = false;
        game_state.second_player_can_castle_queenside = false;
    }
}
