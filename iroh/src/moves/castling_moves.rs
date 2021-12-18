use crate::moves::Move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_castling_moves(available_moves: &mut Vec<Move>, game_state: &GameState) {

    if game_state.is_first_player_turn() {
        let e1 = game_state.board[(File::new(4),Rank::new(0))];

        if game_state.first_player_can_castle_kingside {
            let f1 = game_state.board[(File::new(5), Rank::new(0))];
            let g1 = game_state.board[(File::new(6), Rank::new(0))];
            let h1 = game_state.board[(File::new(7), Rank::new(0))];

            if let (Some(e1_piece), Some(h1_piece)) = (e1, h1) {
                if e1_piece.piece_type == PieceType::King
                    && h1_piece.piece_type == PieceType::Rook
                    && f1.is_none()
                    && g1.is_none() {
                    available_moves.push(Move::Castle(true))
                }
            }
        }

        if game_state.first_player_can_castle_queenside {
            let a1 = game_state.board[(File::new(0), Rank::new(0))];
            let b1 = game_state.board[(File::new(1), Rank::new(0))];
            let c1 = game_state.board[(File::new(2), Rank::new(0))];
            let d1 = game_state.board[(File::new(3), Rank::new(0))];

            if let (Some(a1_piece), Some(e1_piece)) = (a1, e1) {
                if e1_piece.piece_type == PieceType::King
                    && a1_piece.piece_type == PieceType::Rook
                    && b1.is_none()
                    && c1.is_none()
                    && d1.is_none() {
                    available_moves.push(Move::Castle(false))
                }
            }
        }
    } else {
        let e8 = game_state.board[(File::new(4),Rank::new(7))];

        if game_state.second_player_can_castle_kingside {
            let f8 = game_state.board[(File::new(5), Rank::new(7))];
            let g8 = game_state.board[(File::new(6), Rank::new(7))];
            let h8 = game_state.board[(File::new(7), Rank::new(7))];

            if let (Some(e8_piece), Some(h8_piece)) = (e8, h8) {
                if e8_piece.piece_type == PieceType::King
                    && h8_piece.piece_type == PieceType::Rook
                    && f8.is_none()
                    && g8.is_none() {
                    available_moves.push(Move::Castle(true))
                }
            }
        }

        if game_state.second_player_can_castle_queenside {
            let a8 = game_state.board[(File::new(0), Rank::new(7))];
            let b8 = game_state.board[(File::new(1), Rank::new(7))];
            let c8 = game_state.board[(File::new(2), Rank::new(7))];
            let d8 = game_state.board[(File::new(3), Rank::new(7))];

            if let (Some(a8_piece), Some(e8_piece)) = (a8, e8) {
                if e8_piece.piece_type == PieceType::King
                    && a8_piece.piece_type == PieceType::Rook
                    && b8.is_none()
                    && c8.is_none()
                    && d8.is_none() {
                    available_moves.push(Move::Castle(false))
                }
            }
        }
    }
}
