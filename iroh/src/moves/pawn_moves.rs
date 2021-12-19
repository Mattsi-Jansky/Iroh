use crate::moves::Move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_pawn_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank)) {
    let ahead_rank = (if game_state.is_first_player_turn { pawn.2.transform(1) } else { pawn.2.transform(-1) })
        .expect("Pawn is never on back rank, because of promotion");
    let ahead_rank_is_last_rank = if game_state.is_first_player_turn {ahead_rank == 7} else {ahead_rank == 0};
    let is_on_starting_rank = if game_state.is_first_player_turn {pawn.2 == 1} else {pawn.2 == 6};

    if game_state.board[(pawn.1, ahead_rank)].is_none() {
        if ahead_rank_is_last_rank {
            generate_promotion_moves(game_state, available_moves, pawn);
        } else {
            available_moves.push(Move::PawnMove { 0: (pawn.1, pawn.2), 1: ahead_rank });
        }
    }

    if is_on_starting_rank {
        generate_double_move(game_state, available_moves, pawn, ahead_rank)
    }
    generate_attack_moves(game_state, available_moves, pawn, ahead_rank)
}

fn generate_double_move(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), ahead_rank: Rank) {
    let ahead_twice_rank = if game_state.is_first_player_turn { pawn.2 + 2 } else { pawn.2 - 2 };
    if game_state.board[(pawn.1, ahead_rank)].is_none()
        && game_state.board[(pawn.1, ahead_twice_rank)].is_none() {
        available_moves.push(Move::PawnMove { 0: (pawn.1, pawn.2), 1: ahead_twice_rank });
    }
}

fn generate_attack_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), ahead_rank: Rank) {
    let left_file = pawn.1.transform(-1);
    let right_file = pawn.1.transform(1);

    generate_attack_move(game_state, available_moves, pawn, ahead_rank, left_file);
    generate_attack_move(game_state, available_moves, pawn, ahead_rank, right_file);
}

fn generate_attack_move(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), ahead_rank: Rank, file: Option<File>) {
    if let Some(file) = file {
        if let Some(target_piece) = game_state.board[(file, ahead_rank)] {
            if target_piece.is_owned_by_first_player != game_state.is_first_player_turn {
                available_moves.push(Move::PawnAttackMove { 0: pawn.1, 1: (file, ahead_rank) })
            }
        }
    }
}

fn generate_promotion_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank)) {
    available_moves.push(Move::PawnPromotion {
        0: pawn.1,
        1: game_state.is_first_player_turn,
        2: PieceType::Queen
    });
    available_moves.push(Move::PawnPromotion {
        0: pawn.1,
        1: game_state.is_first_player_turn,
        2: PieceType::Knight
    });
    available_moves.push(Move::PawnPromotion {
        0: pawn.1,
        1: game_state.is_first_player_turn,
        2: PieceType::Bishop
    });
    available_moves.push(Move::PawnPromotion {
        0: pawn.1,
        1: game_state.is_first_player_turn,
        2: PieceType::Rook
    });
}
