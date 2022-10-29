use crate::moves::Move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn generate_pawn_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), is_for_first_player: bool) {
    let ahead_rank = (if is_for_first_player { pawn.2.transform(1) } else { pawn.2.transform(-1) })
        .expect("Pawn is never on back rank, because of promotion");
    let ahead_rank_is_last_rank = if is_for_first_player {ahead_rank == 7} else {ahead_rank == 0};
    let is_on_starting_rank = if is_for_first_player {pawn.2 == 1} else {pawn.2 == 6};

    if game_state.board[(pawn.1, ahead_rank)].is_none() {
        if ahead_rank_is_last_rank {
            generate_promotion_moves(game_state, available_moves, pawn, is_for_first_player);
        } else {
            available_moves.push(Move::PawnMove((pawn.1, pawn.2), ahead_rank));
        }
    }

    if is_on_starting_rank {
        generate_double_move(game_state, available_moves, pawn, ahead_rank, is_for_first_player)
    }
    generate_attack_moves(game_state, available_moves, pawn, ahead_rank, is_for_first_player)
}

fn generate_double_move(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), ahead_rank: Rank, is_for_first_player: bool) {
    let ahead_twice_rank = if is_for_first_player { pawn.2 + 2 } else { pawn.2 - 2 };
    if game_state.board[(pawn.1, ahead_rank)].is_none()
        && game_state.board[(pawn.1, ahead_twice_rank)].is_none() {
        available_moves.push(Move::PawnMove((pawn.1, pawn.2),ahead_twice_rank));
    }
}

fn generate_attack_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), ahead_rank: Rank, is_for_first_player: bool) {
    let left_file = pawn.1.transform(-1);
    let right_file = pawn.1.transform(1);

    generate_attack_move(game_state, available_moves, pawn, ahead_rank, left_file, is_for_first_player);
    generate_attack_move(game_state, available_moves, pawn, ahead_rank, right_file, is_for_first_player);
}

fn generate_attack_move(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), ahead_rank: Rank, file: Option<File>, is_for_first_player: bool) {
    if let Some(file) = file {
        if let Some(target_piece) = game_state.board[(file, ahead_rank)] {
            if target_piece.is_owned_by_first_player != is_for_first_player {
                available_moves.push(Move::PawnAttackMove(pawn.1,(file, ahead_rank)))
            }
        }
    }
}

fn generate_promotion_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank), is_for_first_player: bool) {
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        is_for_first_player,
        PieceType::Queen
    ));
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        is_for_first_player,
        PieceType::Knight
    ));
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        is_for_first_player,
        PieceType::Bishop
    ));
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        is_for_first_player,
        PieceType::Rook
    ));
}
