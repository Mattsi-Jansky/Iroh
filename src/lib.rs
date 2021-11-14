mod pgn;

use pgn::generate_pgn;

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

    pub fn get_available_moves(&self, tile: &str) -> Vec<Move> {
        let mut available_moves = vec![];
        let column = (tile.chars().nth(0).unwrap() as u32) as u8 - 97;
        let row = tile.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

        available_moves.push(Move::RegularMove { 0: (column,row), 1: (column,3) });
        available_moves.push(Move::RegularMove { 0: (column,row), 1: (column,4) });

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

        Ok(ChessGame { sans: new })
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
    RegularMove((u8, u8), (u8, u8))
}
