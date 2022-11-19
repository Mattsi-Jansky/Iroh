use crate::moves::Move;
use crate::state::coordinates::Coordinate;
use crate::state::GameState;
use crate::state::tile::{Tile};

pub struct Memento<'a> {
    last_move: &'a Move,
    captured_piece: Tile,
    is_first_player: bool
}

pub fn resolve_move(requested_move: &Move, game_state: &mut GameState) {
    let is_first_player_turn = game_state.is_first_player_turn;
    let memento = perform_move_for(requested_move, game_state, is_first_player_turn);
    if memento.captured_piece != Tile::EMPTY {
        if is_first_player_turn {game_state.captured_pieces.captured_second_player(memento.captured_piece, game_state.turn_number);}
        else {game_state.captured_pieces.captured_first_player(memento.captured_piece, game_state.turn_number);}
    }
    game_state.next_turn();
}

pub fn perform_move_for<'a>(requested_move: &'a Move, game_state: &mut GameState, is_first_player: bool) -> Memento<'a>  {
    match requested_move {
        Move::PawnMove(from, to) => {
            move_piece(game_state, from, to);
            Memento { last_move: requested_move, captured_piece: Tile::EMPTY, is_first_player }
        }
        Move::RegularMove(from, to, _) => {
            move_piece(game_state, from, to);
            Memento { last_move: requested_move, captured_piece: Tile::EMPTY, is_first_player }
        }
        Move::AttackMove(from, to, _) => {
            let target_tile = game_state.board[to];
            assert!(target_tile.is_occupied(), "Illegal move, no target to attack");

            move_piece(game_state, from, to);
            Memento { last_move: requested_move, captured_piece: target_tile, is_first_player }
        }
        Move::PawnAttackMove(from, to) => {
            let target_tile = game_state.board[(to)];
            assert!(target_tile.is_occupied(), "Illegal move, no target to attack");

            move_piece(game_state, from, to);
            Memento { last_move: requested_move, captured_piece: target_tile, is_first_player }
        }
        Move::PawnPromotion(target, tile) => {
            let from = (if tile.is_owned_by_first_player() {target.south()} else {target.north()})
                .expect("Cannot resolve pawn promotion, given invalid move");
            game_state.board[from] = Tile::EMPTY;
            game_state.board[target] = tile.clone();
            Memento { last_move: requested_move, captured_piece: Tile::EMPTY, is_first_player }
        }
        Move::Castle(is_kingside) => {
            match (is_first_player, *is_kingside) {
                (true, true) => {
                    move_piece(game_state, &Coordinate::E1, &Coordinate::G1);
                    move_piece(game_state, &Coordinate::H1,&Coordinate::F1);
                }
                (true, false) => {
                    move_piece(game_state, &Coordinate::E1, &Coordinate::C1);
                    move_piece(game_state, &Coordinate::A1, &Coordinate::D1);
                }
                (false, true) => {
                    move_piece(game_state, &Coordinate::E8, &Coordinate::G8);
                    move_piece(game_state, &Coordinate::H8, &Coordinate::F8);
                }
                (false, false) => {
                    move_piece(game_state, &Coordinate::E8, &Coordinate::C8);
                    move_piece(game_state, &Coordinate::A8, &Coordinate::D8);
                }
            }
            Memento { last_move: requested_move, captured_piece: Tile::EMPTY, is_first_player }
        }
    }
}

pub fn undo_move_for(Memento {last_move, captured_piece, is_first_player}: Memento, game_state: &mut GameState) {

    match last_move {
        Move::RegularMove(from, to, _) => {
            move_piece(game_state, to, from);
        }
        Move::AttackMove(from, to, _) => {
            move_piece(game_state, to, from);
            game_state.board[to] = captured_piece;
        }
        Move::PawnMove(from, to) => {
            move_piece(game_state, to, from);
        }
        Move::PawnAttackMove(from, to) => {
            move_piece(game_state, to, from);
            game_state.board[to] = captured_piece;
        }
        Move::PawnPromotion(_, _) => {}
        Move::Castle(_) => {}
    }
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

#[cfg(test)]
mod tests {
    use crate::moves::Move::*;
    use super::*;
    
    #[test]
    fn undo_regular_move() {
        let mut state = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let requested_move = RegularMove(Coordinate::G1, Coordinate::F3, Tile::FIRST_KNIGHT);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1", state.generate_fen());

        undo_move_for(memento, &mut state);

        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_attack_move() {
        let mut state = GameState::from_fen("3k4/8/8/8/2p5/3K4/8/8 w - - 0 1");
        let requested_move = AttackMove(Coordinate::D3, Coordinate::C4, Tile::FIRST_KING);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("3k4/8/8/8/2K5/8/8/8 w - - 0 1", state.generate_fen());
        assert_eq!(state.captured_pieces.second_player.len(), 0);

        undo_move_for(memento, &mut state);

        assert_eq!("3k4/8/8/8/2p5/3K4/8/8 w - - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_pawn_move() {
        let mut state = GameState::from_fen("4k3/8/8/2P5/1K6/8/8/8 w - - 0 1");
        let requested_move = PawnMove(Coordinate::C5, Coordinate::C6);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("4k3/8/2P5/8/1K6/8/8/8 w - - 0 1", state.generate_fen());

        undo_move_for(memento, &mut state);

        assert_eq!("4k3/8/8/2P5/1K6/8/8/8 w - - 0 1", state.generate_fen());
    }

    #[test]
    fn undo_pawn_attack() {
        let mut state = GameState::from_fen("4k3/8/3p4/2P5/1K6/8/8/8 w - - 0 1");
        let requested_move = PawnAttackMove(Coordinate::C5, Coordinate::D6);

        let memento = perform_move_for(&requested_move, &mut state, true);

        assert_eq!("4k3/8/3P4/8/1K6/8/8/8 w - - 0 1", state.generate_fen());

        undo_move_for(memento, &mut state);

        assert_eq!("4k3/8/3p4/2P5/1K6/8/8/8 w - - 0 1", state.generate_fen());
    }
}
