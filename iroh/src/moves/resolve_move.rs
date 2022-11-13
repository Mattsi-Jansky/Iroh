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
            move_piece(&mut game_state, from, to);
        }
        Move::AttackMove(from, to, _) => {
            let target_tile = game_state.board[to];
            assert!(target_tile.is_occupied(), "Illegal move, no target to attack");
            if is_first_player {game_state.captured_pieces.captured_second_player(target_tile, game_state.turn_number);}
            else {game_state.captured_pieces.captured_first_player(target_tile, game_state.turn_number);}

            move_piece(&mut game_state, from, to);
        }
        Move::PawnAttackMove(from, to) => {
            let target_tile = game_state.board[(to)];
            assert!(target_tile.is_occupied(), "Illegal move, no target to attack");
            if is_first_player {game_state.captured_pieces.captured_second_player(target_tile, game_state.turn_number);}
            else {game_state.captured_pieces.captured_first_player(target_tile, game_state.turn_number);}

            move_piece(&mut game_state, from, to);
        }
        Move::PawnPromotion(target, tile) => {
            let from = (if tile.is_owned_by_first_player() {target.south()} else {target.north()})
                .expect("Cannot resolve pawn promotion, given invalid move");
            game_state.board[from] = Tile::EMPTY;
            game_state.board[target] = tile.clone()
        }
        Move::Castle(is_kingside) => {
            match (is_first_player, *is_kingside) {
                (true, true) => {
                    move_piece(&mut game_state, &Coordinate::E1, &Coordinate::G1);
                    move_piece(&mut game_state, &Coordinate::H1,&Coordinate::F1);
                }
                (true, false) => {
                    move_piece(&mut game_state, &Coordinate::E1, &Coordinate::C1);
                    move_piece(&mut game_state, &Coordinate::A1, &Coordinate::D1);
                }
                (false, true) => {
                    move_piece(&mut game_state, &Coordinate::E8, &Coordinate::G8);
                    move_piece(&mut game_state, &Coordinate::H8, &Coordinate::F8);
                }
                (false, false) => {
                    move_piece(&mut game_state, &Coordinate::E8, &Coordinate::C8);
                    move_piece(&mut game_state, &Coordinate::A8, &Coordinate::D8);
                }
            }
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
