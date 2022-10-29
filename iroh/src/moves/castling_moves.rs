use crate::moves::Move;
use crate::moves::resolve_move::resolve_move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_castling_moves(available_moves: &mut Vec<Move>, game_state: &GameState, is_for_first_player: bool) {

    if is_for_first_player {
        let e1 = game_state.board[(File::E,Rank::ONE)];

        if game_state.first_player_can_castle_kingside {
            let g1 = game_state.board[(File::G, Rank::ONE)];
            let f1 = game_state.board[(File::F, Rank::ONE)];
            let h1 = game_state.board[(File::H, Rank::ONE)];

            if let (Some(e1_piece), Some(h1_piece)) = (e1, h1) {
                if e1_piece.piece_type == PieceType::King
                    && h1_piece.piece_type == PieceType::Rook
                    && f1.is_none()
                    && g1.is_none() {
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

            if let (Some(a1_piece), Some(e1_piece)) = (a1, e1) {
                if e1_piece.piece_type == PieceType::King
                    && a1_piece.piece_type == PieceType::Rook
                    && b1.is_none()
                    && c1.is_none()
                    && d1.is_none() {
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

            if let (Some(e8_piece), Some(h8_piece)) = (e8, h8) {
                if e8_piece.piece_type == PieceType::King
                    && h8_piece.piece_type == PieceType::Rook
                    && f8.is_none()
                    && g8.is_none() {
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

            if let (Some(a8_piece), Some(e8_piece)) = (a8, e8) {
                if e8_piece.piece_type == PieceType::King
                    && a8_piece.piece_type == PieceType::Rook
                    && b8.is_none()
                    && c8.is_none()
                    && d8.is_none() {
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
        PieceType::King
    );
    let result = resolve_move(&skipped_move, game_state.clone());
    result.is_check(true)
}

fn would_be_check_second_player(target_file: File, game_state: &GameState) -> bool {
    let skipped_move = Move::RegularMove (
        (File::E, Rank::EIGHT),
        (target_file, Rank::EIGHT),
        PieceType::King
    );
    let result = resolve_move(&skipped_move, game_state.clone());
    result.is_check(false)
}
