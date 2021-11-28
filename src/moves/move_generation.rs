use crate::state::coordinates::{File, Rank};
use crate::moves::Move;
use crate::state::GameState;
use crate::state::piece::PieceType;

const STRAIGHT_DYNAMIC_TRANSFORMS: [(isize,isize);4] = [(1,0),(0,1),(-1,0),(0,-1)];
const DIAGONAL_DYNAMIC_TRANSFORMS: [(isize,isize);4] = [(1,1),(1,-1),(-1,1),(-1,-1)];

pub fn generate_moves(game_state: &GameState) -> Vec<Move> {
    let mut available_moves = vec![];

    let pieces = game_state.board.get_all_pieces_belonging_to_player(game_state.is_first_player_turn());
    for piece in pieces {
        match piece.0 {
            PieceType::Pawn => generate_pawn_moves(game_state,&mut available_moves,piece),
            PieceType::Knight => generate_knight_moves(&mut available_moves, piece, game_state),
            PieceType::King => generate_king_moves(&mut available_moves, piece, game_state),
            PieceType::Rook => generate_rook_moves(&mut available_moves, piece, game_state),
            PieceType::Bishop => generate_bishop_moves(&mut available_moves, piece, game_state),
            PieceType::Queen => generate_queen_moves(&mut available_moves, piece, game_state)
        }
    }

    available_moves
}

fn generate_queen_moves(available_moves: &mut Vec<Move>, queen: (PieceType, File, Rank), game_state: &GameState) {
    generate_dynamic_moves(game_state, available_moves, queen, &[STRAIGHT_DYNAMIC_TRANSFORMS, DIAGONAL_DYNAMIC_TRANSFORMS].concat());
}

fn generate_bishop_moves(available_moves: &mut Vec<Move>, bishop: (PieceType, File, Rank), game_state: &GameState) {
    generate_dynamic_moves(game_state, available_moves, bishop, &DIAGONAL_DYNAMIC_TRANSFORMS);
}

fn generate_rook_moves(available_moves: &mut Vec<Move>, rook: (PieceType, File, Rank), game_state: &GameState) {
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
                    if target_piece.is_owned_by_first_player != game_state.is_first_player_turn() {
                        available_moves.push(Move::AttackMove {
                            0: (piece.1, piece.2),
                            1: (file, rank),
                            2: piece.0
                        })
                    }
                } else {
                    available_moves.push(Move::RegularMove {
                        0: (piece.1, piece.2),
                        1: (file, rank),
                        2: piece.0
                    })
                }
            }
            else {
                break;
            }
        }
    };
}

fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank), game_state: &GameState) {
    [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)].map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform,
            game_state) {
            available_moves.push(m)
        }
    });
}

fn generate_king_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank), game_state: &GameState) {
    [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)].map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform,
            game_state) {
            available_moves.push(m)
        }
    });
}

fn generate_static_move_if_legal(piece: (PieceType, File, Rank), transformation: (isize, isize), game_state: &GameState) -> Option<Move> {
    let target_file = piece.1.transform(transformation.0);
    let target_rank = piece.2.transform(transformation.1);

    if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
        if let Some(target_piece) = game_state.board[(target_file, target_rank)] {
            if target_piece.is_owned_by_first_player != game_state.is_first_player_turn() {
                Some(Move::AttackMove {
                    0: (piece.1, piece.2),
                    1: (target_file, target_rank),
                    2: piece.0
                })
            }
            else {None}
        } else {
            Some(Move::RegularMove {
                0: (piece.1, piece.2),
                1: (target_file, target_rank),
                2: piece.0
            })
        }
    } else {None}
}

fn generate_pawn_moves(game_state: &GameState, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank)) {
    let left_file = pawn.1.transform(-1);
    let right_file = pawn.1.transform(1);
    let ahead_rank = (if game_state.is_first_player_turn() { pawn.2.transform(1) } else { pawn.2.transform(-1) })
        .expect("Pawn is never on back rank, because of promotion");
    let ahead_twice_rank = if game_state.is_first_player_turn() { pawn.2 + 2 } else { pawn.2 - 2 };

    if !game_state.board[(pawn.1, ahead_rank)].is_some() {
        available_moves.push(Move::PawnMove { 0: pawn.1, 1: ahead_rank });
    }
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: ahead_twice_rank });

    if let Some(left_file) = left_file {
        if let Some(target_piece) = game_state.board[(left_file, ahead_rank)] {
            if target_piece.is_owned_by_first_player != game_state.is_first_player_turn() {
                available_moves.push(Move::PawnAttackMove { 0: pawn.1, 1: (left_file, ahead_rank) })
            }
        }
    }
    if let Some(right_file) = right_file {
        if let Some(target_piece) = game_state.board[(right_file, ahead_rank)] {
            if target_piece.is_owned_by_first_player != game_state.is_first_player_turn() {
                available_moves.push(Move::PawnAttackMove { 0: pawn.1, 1: (right_file, ahead_rank) })
            }
        }
    }
}
