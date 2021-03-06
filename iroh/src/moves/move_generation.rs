use crate::moves::{dynamic_moves, Move, pawn_moves, static_moves};
use crate::moves::castling_moves::generate_castling_moves;
use crate::moves::resolve_move::resolve_move;
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_moves(game_state: &GameState) -> Vec<Move> {
    let mut available_moves = vec![];

    let pieces = game_state.board.get_all_pieces_belonging_to_player(game_state.is_first_player_turn);
    for piece in pieces {
        match piece.0 {
            PieceType::Pawn => pawn_moves::generate_pawn_moves(game_state, &mut available_moves, piece),
            PieceType::Knight => static_moves::generate_knight_moves(&mut available_moves, piece, game_state),
            PieceType::King => static_moves::generate_king_moves(&mut available_moves, piece, game_state),
            PieceType::Rook => dynamic_moves::generate_rook_moves(&mut available_moves, piece, game_state),
            PieceType::Bishop => dynamic_moves::generate_bishop_moves(&mut available_moves, piece, game_state),
            PieceType::Queen => dynamic_moves::generate_queen_moves(&mut available_moves, piece, game_state)
        }
    }

    generate_castling_moves(&mut available_moves, game_state);
    available_moves = remove_moves_that_result_in_check(available_moves, game_state);

    available_moves
}

fn remove_moves_that_result_in_check(available_moves: Vec<Move>, game_state: &GameState) -> Vec<Move>{
    available_moves.into_iter().filter(|requested_move| {
        let new_game_state = resolve_move(requested_move, game_state.clone());

        !new_game_state.is_check(game_state.is_first_player_turn)
    }).collect()
}
