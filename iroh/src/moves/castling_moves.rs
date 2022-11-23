use crate::moves::Move;
use crate::moves::resolve_move::{perform_move_for, undo_move};
use crate::state::coordinates::Coordinate;
use crate::state::GameState;
use crate::state::tile::{Tile};

pub fn generate_castling_moves(available_moves: &mut Vec<Move>, game_state: &mut GameState, is_for_first_player: bool) {

    if is_for_first_player {
        let e1 = game_state.board[Coordinate::E1];

        if game_state.first_player_can_castle_kingside {
            let g1 = game_state.board[Coordinate::G1];
            let f1 = game_state.board[Coordinate::F1];
            let h1 = game_state.board[Coordinate::H1];

            if e1.is_occupied() && h1.is_occupied() {
                if e1 == Tile::FIRST_KING
                    && h1 == Tile::FIRST_ROOK
                    && !f1.is_occupied()
                    && !g1.is_occupied() {
                    let f1_would_be_check = would_be_check_first_player(Coordinate::F1, game_state);
                    //Do not need to check g1 because castling there would result in check anyway

                    if !f1_would_be_check {
                        available_moves.push(Move::Castle(true))
                    }
                }
            }
        }

        if game_state.first_player_can_castle_queenside {
            let a1 = game_state.board[Coordinate::A1];
            let b1 = game_state.board[Coordinate::B1];
            let c1 = game_state.board[Coordinate::C1];
            let d1 = game_state.board[Coordinate::D1];

            if a1.is_occupied() && e1.is_occupied() {
                if e1 == Tile::FIRST_KING
                    && a1 == Tile::FIRST_ROOK
                    && !b1.is_occupied()
                    && !c1.is_occupied()
                    && !d1.is_occupied() {
                    let d1_would_be_check = would_be_check_first_player(Coordinate::D1, game_state);
                    //Do not need to check c1 because castling there would result in check anyway

                    if !d1_would_be_check {
                        available_moves.push(Move::Castle(false))
                    }
                }
            }
        }
    } else {
        let e8 = game_state.board[Coordinate::E8];

        if game_state.second_player_can_castle_kingside {
            let f8 = game_state.board[Coordinate::F8];
            let g8 = game_state.board[Coordinate::G8];
            let h8 = game_state.board[Coordinate::H8];

            if e8.is_occupied() && h8.is_occupied() {
                if e8 == Tile::SECOND_KING
                    && h8 == Tile::SECOND_ROOK
                    && !f8.is_occupied()
                    && !g8.is_occupied() {
                    let f8_would_be_check = would_be_check_second_player(Coordinate::F8, game_state);
                    //Do not need to check g8 because castling there would result in check anyway

                    if !f8_would_be_check {
                        available_moves.push(Move::Castle(true))
                    }
                }
            }
        }

        if game_state.second_player_can_castle_queenside {
            let a8 = game_state.board[Coordinate::A8];
            let b8 = game_state.board[Coordinate::B8];
            let c8 = game_state.board[Coordinate::C8];
            let d8 = game_state.board[Coordinate::D8];

            if a8.is_occupied() && e8.is_occupied() {
                if e8 == Tile::SECOND_KING
                    && a8 == Tile::SECOND_ROOK
                    && !b8.is_occupied()
                    && !c8.is_occupied()
                    && !d8.is_occupied() {
                    let d8_would_be_check = would_be_check_second_player(Coordinate::D8, game_state);
                    //Do not need to check c8 because castling there would result in check anyway

                    if !d8_would_be_check {
                        available_moves.push(Move::Castle(false))
                    }
                }
            }
        }
    }
}

fn would_be_check_first_player(target: Coordinate, game_state: &mut GameState) -> bool {
    let skipped_move = Move::RegularMove (
        Coordinate::E1,
        target,
        Tile::FIRST_KING
    );
    let memento = perform_move_for(&skipped_move, game_state, true);
    let result = game_state.is_check(true);
    undo_move(memento, game_state);
    result
}

fn would_be_check_second_player(target: Coordinate, game_state: &mut GameState) -> bool {
    let skipped_move = Move::RegularMove (
        Coordinate::E8,
        target,
        Tile::SECOND_KING
    );
    let memento = perform_move_for(&skipped_move, game_state, false);
    let result = game_state.is_check(false);
    undo_move(memento, game_state);
    result
}
