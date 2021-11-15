mod pgn;

use pgn::{generate_pgn, parse_san};

pub struct ChessGame {
    sans: Vec<String>,
}

#[derive(Debug)]
pub struct ChessGameError {

}

impl ChessGame {
    pub fn new() -> ChessGame {
        ChessGame { sans: vec![] }
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        let mut available_moves = vec![];

        for column in 0..=7 {
            available_moves.push(Move::PawnMove { 0: column, 1: (column, 3) });
            available_moves.push(Move::PawnMove { 0: column, 1: (column, 4) });
        }

        available_moves
    }

    pub fn get_pgn(&self) -> String {
        if self.sans.is_empty() {
            return String::new()
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
            Err(ChessGameError {})
        }
        else {
            Ok(ChessGame { sans: new })
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
    RegularMove((u8, u8), (u8, u8)),
    PawnMove(u8,(u8,u8))
}
