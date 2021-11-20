use crate::board::Board;
use crate::moves::Move;
use crate::piece::PieceType;

pub fn generate_moves(is_first_player_turn: bool, board: &Board) -> Vec<Move> {
    let mut available_moves = vec![];

    let pawns = board.get_all(PieceType::Pawn, is_first_player_turn);
    for pawn in pawns {
        let move_once_row = if is_first_player_turn {pawn.2 + 1} else {pawn.2 - 1};
        let move_twice_row = if is_first_player_turn {pawn.2 + 2} else {pawn.2 - 2};
        available_moves.push(Move::PawnMove {0: pawn.1, 1: (pawn.1, move_once_row)});
        available_moves.push(Move::PawnMove {0: pawn.1, 1: (pawn.1, move_twice_row)});
    }

    let knights = board.get_all(PieceType::Knight, is_first_player_turn);

    for knight in knights {
        regular_move_if_legal((knight.1,knight.2),(1,2))
            .map(|m| available_moves.push(m));
        regular_move_if_legal((knight.1,knight.2),(2,1))
            .map(|m| available_moves.push(m));
        regular_move_if_legal((knight.1,knight.2),(-1,-2))
            .map(|m| available_moves.push(m));
        regular_move_if_legal((knight.1,knight.2),(-2,-1))
            .map(|m| available_moves.push(m));

        regular_move_if_legal((knight.1,knight.2),(1,-2))
            .map(|m| available_moves.push(m));
        regular_move_if_legal((knight.1,knight.2),(2,-1))
            .map(|m| available_moves.push(m));
        regular_move_if_legal((knight.1,knight.2),(-1,2))
            .map(|m| available_moves.push(m));
        regular_move_if_legal((knight.1,knight.2),(-2,1))
            .map(|m| available_moves.push(m));
    }

    available_moves
}

fn regular_move_if_legal(starting_position: (usize, usize), transformation: (isize, isize)) -> Option<Move> {

    Some(Move::RegularMove {
        0: starting_position,
        1: (
            ((starting_position.0 as isize) + transformation.0) as usize,
            ((starting_position.1 as isize) + transformation.1) as usize
        )
    })
}