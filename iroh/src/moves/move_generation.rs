use crate::moves::{dynamic_moves, Move, pawn_moves, static_moves};
use crate::moves::castling_moves::generate_castling_moves;
use crate::moves::resolve_move::{resolve_move_for};
use crate::state::GameState;
use crate::state::piece::{Piece};

pub fn generate_moves(game_state: &GameState, is_for_first_player: bool) -> Vec<Move> {
    let mut available_moves = vec![];

    let pieces = game_state.board.get_all_pieces_belonging_to_player(is_for_first_player);
    for piece in pieces {
        match piece.0 {
            Piece::FIRST_PAWN | Piece::SECOND_PAWN => pawn_moves::generate_pawn_moves(game_state, &mut available_moves, piece, is_for_first_player),
            Piece::FIRST_KNIGHT | Piece::SECOND_KNIGHT => static_moves::generate_knight_moves(&mut available_moves, piece, game_state, is_for_first_player),
            Piece::FIRST_KING | Piece::SECOND_KING => static_moves::generate_king_moves(&mut available_moves, piece, game_state, is_for_first_player),
            Piece::FIRST_ROOK | Piece::SECOND_ROOK => dynamic_moves::generate_rook_moves(&mut available_moves, piece, game_state, is_for_first_player),
            Piece::FIRST_BISHOP | Piece::SECOND_BISHOP => dynamic_moves::generate_bishop_moves(&mut available_moves, piece, game_state, is_for_first_player),
            Piece::FIRST_QUEEN | Piece::SECOND_QUEEN => dynamic_moves::generate_queen_moves(&mut available_moves, piece, game_state, is_for_first_player),
            _ => { panic!("This should never happen - piece is not a valid recognised chesspiece") }
        }
    }

    generate_castling_moves(&mut available_moves, game_state, is_for_first_player);
    available_moves = remove_moves_that_result_in_check(available_moves, game_state, is_for_first_player);

    available_moves
}

fn remove_moves_that_result_in_check(available_moves: Vec<Move>, game_state: &GameState, is_for_first_player: bool) -> Vec<Move>{
    available_moves.into_iter().filter(|requested_move| {
        let new_game_state = resolve_move_for(requested_move, game_state.clone(), is_for_first_player);

        !new_game_state.is_check(is_for_first_player)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_moves_for_either_player() {
        let state = GameState::from_fen("8/2nk4/3q4/8/2P1P3/2K5/8/8 w - - 0 1");

        let n_moves_for_first_player = generate_moves(&state, true)
            .iter().count();
        let n_moves_for_second_player = generate_moves(&state, false)
            .iter().count();

        assert_eq!(5, n_moves_for_first_player);
        assert_eq!(33, n_moves_for_second_player);
    }

    #[test]
    fn generate_castling_moves_for_either_player() {
        let state = GameState::from_fen("rnbqk2r/ppppppbp/5np1/8/8/1P3NP1/P1PPPP1P/RNBQKB1R w KQkq - 0 1");

        let n_castle_moves_for_first_player = generate_moves(&state, true)
            .iter().filter(|m| matches!(m, Move::Castle(_))).count();
        let n_castle_moves_for_second_player = generate_moves(&state, false)
            .iter().filter(|m| matches!(m, Move::Castle(_))).count();

        assert_eq!(0, n_castle_moves_for_first_player);
        assert_eq!(1, n_castle_moves_for_second_player);
    }
}
