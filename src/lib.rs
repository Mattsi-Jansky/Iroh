mod pgn;
mod piece;
mod board;
mod fen;

use pgn::{generate_pgn, parse_san};
use crate::board::Board;
use crate::piece::ChessPieceType;

pub struct ChessGame {
    sans: Vec<String>,
    board: Board,
    is_first_player_turn: bool,
}

#[derive(Debug)]
pub struct ChessGameError {

}

fn transform_chess_board(requested_move: Move, mut board: Board) -> Board {
    match requested_move {
        Move::PawnMove(from_column,(to_column,to_row)) => {
            if from_column == to_column {
                let from = (from_column,to_row-1);
                let to = (to_column,to_row);
                let piece = board[from];
                board[from] = None;
                board[to] = piece;
            } else { todo!("Pawn attack moves")}
        }
        _ => panic!("Unknown move")
    }

    board
}

impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame { sans: vec![], board: Board::new(), is_first_player_turn: true }
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        let mut available_moves = vec![];

        let pawns = self.board.get_all(ChessPieceType::Pawn, self.is_first_player_turn);
        for pawn in pawns {
            let move_once_row = if self.is_first_player_turn {pawn.2 + 1} else {pawn.2 - 1};
            let move_twice_row = if self.is_first_player_turn {pawn.2 + 2} else {pawn.2 - 2};
            available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_once_row) });
            available_moves.push(Move::PawnMove { 0: pawn.1, 1: (pawn.1, move_twice_row) });
        }

        available_moves
    }

    pub fn get_pgn(&self) -> String {
        if self.sans.is_empty() {
            String::new()
        }
        else {
            generate_pgn(&self.sans)
        }
    }

    pub fn make_move(self, san: &str) -> Result<ChessGame,ChessGameError> {
        let mut new = self.sans.clone();
        new.push(String::from(san));

        let requested_move = parse_san(san);
        let possible_moves = self.get_available_moves();

        if !possible_moves.contains(&requested_move) {
            println!("Cannot get move {:?} from {:?}", requested_move, possible_moves);
            Err(ChessGameError {})
        }
        else {
            let board = transform_chess_board(requested_move, self.board);
            Ok(ChessGame { sans: new, board, is_first_player_turn: !self.is_first_player_turn })
        }
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Move {
    RegularMove((usize, usize), (usize, usize)),
    PawnMove(usize,(usize,usize))
}
