use crate::pgn::{generate_pgn, parse_san};
use crate::board::Board;
use crate::error::ChessGameError;
use crate::move_generation::generate_moves;
use crate::moves::{Move};
use crate::piece::ChessPieceType;
use crate::resolve_move::resolve_move;

pub struct ChessGame {
    sans: Vec<String>,
    board: Board,
    is_first_player_turn: bool,
}

impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame { sans: vec![], board: Board::new(), is_first_player_turn: true }
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        generate_moves(self.is_first_player_turn, &self.board)
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
            let board = resolve_move(requested_move, self.board);
            Ok(ChessGame { sans: new, board, is_first_player_turn: !self.is_first_player_turn })
        }
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}
