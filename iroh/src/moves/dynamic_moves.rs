use crate::moves::{DIAGONAL_DYNAMIC_TRANSFORMS, Move, STRAIGHT_DYNAMIC_TRANSFORMS};
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_queen_moves(available_moves: &mut Vec<Move>, queen: (PieceType, File, Rank), game_state: &GameState) {
    generate_dynamic_moves(game_state, available_moves, queen, &[STRAIGHT_DYNAMIC_TRANSFORMS, DIAGONAL_DYNAMIC_TRANSFORMS].concat());
}

pub fn generate_bishop_moves(available_moves: &mut Vec<Move>, bishop: (PieceType, File, Rank), game_state: &GameState) {
    generate_dynamic_moves(game_state, available_moves, bishop, &DIAGONAL_DYNAMIC_TRANSFORMS);
}

pub fn generate_rook_moves(available_moves: &mut Vec<Move>, rook: (PieceType, File, Rank), game_state: &GameState) {
    generate_dynamic_moves(game_state, available_moves, rook, &STRAIGHT_DYNAMIC_TRANSFORMS);
}

fn generate_dynamic_moves(game_state: &GameState, available_moves: &mut Vec<Move>, piece: (PieceType, File, Rank), transformations: &[(isize, isize)]) {
    for transform in transformations {
        let (mut file, mut rank) = (Some(piece.1), Some(piece.2));
        loop {
            file = file.unwrap().transform(transform.0);
            rank = rank.unwrap().transform(transform.1);
            if let(Some(file), Some(rank)) = (file,rank) {
                if let Some(target_piece) = game_state.board[(file, rank)] {
                    if target_piece.is_owned_by_first_player != game_state.is_first_player_turn {
                        available_moves.push(Move::AttackMove (
                            (piece.1, piece.2),
                            (file, rank),
                            piece.0
                        ))
                    }
                    break;
                } else {
                    available_moves.push(Move::RegularMove (
                        (piece.1, piece.2),
                        (file, rank),
                        piece.0
                    ))
                }
            }
            else {
                break;
            }
        }
    };
}
