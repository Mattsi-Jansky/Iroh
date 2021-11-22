use crate::board::Board;
use crate::coordinates::{File, Rank};
use crate::moves::Move;
use crate::piece::PieceType;

const STRAIGHT_DYNAMIC_TRANSFORMS: [(isize,isize);4] = [(1,0),(0,1),(-1,0),(0,-1)];
const DIAGONAL_DYNAMIC_TRANSFORMS: [(isize,isize);4] = [(1,1),(1,-1),(-1,1),(-1,-1)];

pub fn generate_moves(is_first_player_turn: bool, board: &Board) -> Vec<Move> {
    let mut available_moves = vec![];

    let pieces = board.get_all_pieces_belonging_to_player(is_first_player_turn);
    for piece in pieces {
        match piece.0 {
            PieceType::Pawn => generate_pawn_moves(is_first_player_turn,&mut available_moves,piece),
            PieceType::Knight => generate_knight_moves(&mut available_moves, piece),
            PieceType::King => generate_king_moves(&mut available_moves, piece),
            PieceType::Rook => generate_rook_moves(&mut available_moves, piece, board),
            PieceType::Bishop => generate_bishop_moves(&mut available_moves, piece, board),
            PieceType::Queen => generate_queen_moves(&mut available_moves, piece, board)
        }
    }

    available_moves
}

fn generate_queen_moves(available_moves: &mut Vec<Move>, queen: (PieceType, File, Rank), board: &Board) {
    generate_dynamic_moves(board, available_moves, queen, &[STRAIGHT_DYNAMIC_TRANSFORMS, DIAGONAL_DYNAMIC_TRANSFORMS].concat());
}

fn generate_bishop_moves(available_moves: &mut Vec<Move>, bishop: (PieceType, File, Rank), board: &Board) {
    generate_dynamic_moves(board, available_moves, bishop, &DIAGONAL_DYNAMIC_TRANSFORMS);
}

fn generate_rook_moves(available_moves: &mut Vec<Move>, rook: (PieceType, File, Rank), board: &Board) {
    generate_dynamic_moves(board, available_moves, rook,&STRAIGHT_DYNAMIC_TRANSFORMS);
}

fn generate_dynamic_moves(board: &Board, available_moves: &mut Vec<Move>, piece: (PieceType, File, Rank), transformations: &[(isize, isize)]) {
    for transform in transformations {
        let (mut file, mut rank) = (Some(piece.1), Some(piece.2));
        loop {
            file = file.unwrap().transform(transform.0);
            rank = rank.unwrap().transform(transform.1);
            if let(Some(file), Some(rank)) = (file,rank) {
                if board[(file,rank)].is_some() {
                    available_moves.push(Move::AttackMove {
                        0: (piece.1, piece.2),
                        1: (file, rank),
                        2: piece.0
                    })
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

fn generate_knight_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank)) {
    [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)].map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform) {
            available_moves.push(m)
        }
    });
}

fn generate_king_moves(available_moves: &mut Vec<Move>, knight: (PieceType, File, Rank)) {
    [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)].map(|transform| {
        if let Some(m) = generate_static_move_if_legal(
            knight,
            transform) {
            available_moves.push(m)
        }
    });
}

fn generate_pawn_moves(is_first_player_turn: bool, available_moves: &mut Vec<Move>, pawn: (PieceType, File, Rank)) {
    let move_once_rank = if is_first_player_turn { pawn.2 + 1 } else { pawn.2 - 1 };
    let move_twice_rank = if is_first_player_turn { pawn.2 + 2 } else { pawn.2 - 2 };
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_once_rank) });
    available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_twice_rank) });
}

fn generate_static_move_if_legal(piece: (PieceType, File, Rank), transformation: (isize, isize)) -> Option<Move> {
    let target_file = piece.1.transform(transformation.0);
    let target_rank = piece.2.transform(transformation.1);

    if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
        Some(Move::RegularMove {
            0: (piece.1, piece.2),
            1: (target_file,target_rank),
            2: piece.0
        })
    } else {None}
}
