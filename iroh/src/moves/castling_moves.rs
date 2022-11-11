use crate::moves::Move;
use crate::moves::resolve_move::resolve_move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::tile::{Tile};

pub fn generate_castling_moves(available_moves: &mut Vec<Move>, game_state: &GameState, is_for_first_player: bool) {

    if is_for_first_player {
        let e1 = game_state.board[(File::E,Rank::ONE)];

        if game_state.first_player_can_castle_kingside {
            let g1 = game_state.board[(File::G, Rank::ONE)];
            let f1 = game_state.board[(File::F, Rank::ONE)];
            let h1 = game_state.board[(File::H, Rank::ONE)];

            if e1.is_occupied() && h1.is_occupied() {
                if e1 == Tile::FIRST_KING
                    && h1 == Tile::FIRST_ROOK
                    && !f1.is_occupied()
                    && !g1.is_occupied() {
                    let f1_would_be_check = would_be_check_first_player(File::F, game_state);
                    //Do not need to check g1 because castling there would result in check anyway

                    if !f1_would_be_check {
                        available_moves.push(Move::Castle(true))
                    }
                }
            }
        }

        if game_state.first_player_can_castle_queenside {
            let a1 = game_state.board[(File::A, Rank::ONE)];
            let b1 = game_state.board[(File::B, Rank::ONE)];
            let c1 = game_state.board[(File::C, Rank::ONE)];
            let d1 = game_state.board[(File::D, Rank::ONE)];

            if a1.is_occupied() && e1.is_occupied() {
                if e1 == Tile::FIRST_KING
                    && a1 == Tile::FIRST_ROOK
                    && !b1.is_occupied()
                    && !c1.is_occupied()
                    && !d1.is_occupied() {
                    let d1_would_be_check = would_be_check_first_player(File::D, game_state);
                    //Do not need to check c1 because castling there would result in check anyway

                    if !d1_would_be_check {
                        available_moves.push(Move::Castle(false))
                    }
                }
            }
        }
    } else {
        let e8 = game_state.board[(File::E,Rank::EIGHT)];

        if game_state.second_player_can_castle_kingside {
            let f8 = game_state.board[(File::F, Rank::EIGHT)];
            let g8 = game_state.board[(File::G, Rank::EIGHT)];
            let h8 = game_state.board[(File::H, Rank::EIGHT)];

            if e8.is_occupied() && h8.is_occupied() {
                if e8 == Tile::SECOND_KING
                    && h8 == Tile::SECOND_ROOK
                    && !f8.is_occupied()
                    && !g8.is_occupied() {
                    let f8_would_be_check = would_be_check_second_player(File::F, game_state);
                    //Do not need to check g8 because castling there would result in check anyway

                    if !f8_would_be_check {
                        available_moves.push(Move::Castle(true))
                    }
                }
            }
        }

        if game_state.second_player_can_castle_queenside {
            let a8 = game_state.board[(File::A, Rank::EIGHT)];
            let b8 = game_state.board[(File::B, Rank::EIGHT)];
            let c8 = game_state.board[(File::C, Rank::EIGHT)];
            let d8 = game_state.board[(File::D, Rank::EIGHT)];

            if a8.is_occupied() && e8.is_occupied() {
                if e8 == Tile::SECOND_KING
                    && a8 == Tile::SECOND_ROOK
                    && !b8.is_occupied()
                    && !c8.is_occupied()
                    && !d8.is_occupied() {
                    let d8_would_be_check = would_be_check_second_player(File::D, game_state);
                    //Do not need to check c8 because castling there would result in check anyway

                    if !d8_would_be_check {
                        available_moves.push(Move::Castle(false))
                    }
                }
            }
        }
    }
}

fn would_be_check_first_player(target_file: File, game_state: &GameState) -> bool {
    let skipped_move = Move::RegularMove (
        (File::E, Rank::ONE),
        (target_file, Rank::ONE),
        Tile::FIRST_KING
    );
    let result = resolve_move(&skipped_move, game_state.clone());
    result.is_check(true)
}

fn would_be_check_second_player(target_file: File, game_state: &GameState) -> bool {
    let skipped_move = Move::RegularMove (
        (File::E, Rank::EIGHT),
        (target_file, Rank::EIGHT),
        Tile::SECOND_KING
    );
    let result = resolve_move(&skipped_move, game_state.clone());
    result.is_check(false)
}
