use crate::moves::Move;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::{Piece};

pub fn generate_pawn_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (Piece, File, Rank), is_for_first_player: bool) {
    let ahead_rank = (if is_for_first_player { pawn.2.transform(1) } else { pawn.2.transform(-1) })
        .expect("Pawn is never on back rank, because of promotion");
    let ahead_rank_is_last_rank = if is_for_first_player {ahead_rank == 7} else {ahead_rank == 0};
    let is_on_starting_rank = if is_for_first_player {pawn.2 == 1} else {pawn.2 == 6};

    if !game_state.board[(pawn.1, ahead_rank)].is_occupied() {
        if ahead_rank_is_last_rank {
            generate_promotion_moves(available_moves, pawn, is_for_first_player);
        } else {
            available_moves.push(Move::PawnMove((pawn.1, pawn.2), ahead_rank));
        }
    }

    if is_on_starting_rank {
        generate_double_move(game_state, available_moves, pawn, ahead_rank, is_for_first_player)
    }
    generate_attack_moves(game_state, available_moves, pawn, ahead_rank, is_for_first_player)
}

fn generate_double_move(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (Piece, File, Rank), ahead_rank: Rank, is_for_first_player: bool) {
    let ahead_twice_rank = if is_for_first_player { pawn.2 + 2 } else { pawn.2 - 2 };
    if !game_state.board[(pawn.1, ahead_rank)].is_occupied()
        && !game_state.board[(pawn.1, ahead_twice_rank)].is_occupied() {
        available_moves.push(Move::PawnMove((pawn.1, pawn.2),ahead_twice_rank));
    }
}

fn generate_attack_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (Piece, File, Rank), ahead_rank: Rank, is_for_first_player: bool) {
    let left_file = pawn.1.transform(-1);
    let right_file = pawn.1.transform(1);

    generate_attack_move(game_state, available_moves, pawn, ahead_rank, left_file, is_for_first_player);
    generate_attack_move(game_state, available_moves, pawn, ahead_rank, right_file, is_for_first_player);
}

fn generate_attack_move(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (Piece, File, Rank), ahead_rank: Rank, file: Option<File>, is_for_first_player: bool) {
    if let Some(file) = file {
        let tile = game_state.board[(file, ahead_rank)];
        if tile.is_occupied() {
            if tile.is_owned_by_first_player() != is_for_first_player {
                available_moves.push(Move::PawnAttackMove(pawn.1,(file, ahead_rank)))
            }
        }
    }
}

fn generate_promotion_moves(available_moves: &mut Vec<Move>, pawn: (Piece, File, Rank), is_for_first_player: bool) {
    available_moves.push(Move::PawnPromotion (
        pawn.1,
    if is_for_first_player { Piece::FIRST_QUEEN } else { Piece::SECOND_QUEEN }
    ));
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        if is_for_first_player { Piece::FIRST_ROOK } else { Piece::SECOND_ROOK }
    ));
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        if is_for_first_player { Piece::FIRST_BISHOP } else { Piece::SECOND_BISHOP }
    ));
    available_moves.push(Move::PawnPromotion (
        pawn.1,
        if is_for_first_player { Piece::FIRST_KNIGHT } else { Piece::SECOND_KNIGHT }
    ));
}
