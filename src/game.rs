use std::collections::HashMap;
use crate::serialisers::pgn::generate_pgn;
use crate::serialisers::san::parse_san;
use crate::board::Board;
use crate::error::IllegalMoveError;
use crate::moves::move_generation::generate_moves;
use crate::moves::{Move};
use crate::moves::resolve_move::resolve_move;

pub struct Game {
    sans: Vec<String>,
    board: Board,
    is_first_player_turn: bool,
}

impl Game {
    pub fn new() -> Game {
        Game { sans: vec![], board: Board::new(), is_first_player_turn: true }
    }

    pub fn from_fen(fen: &str) -> Game {
        Game { sans: vec![], board: Board::from_fen(fen), is_first_player_turn: true }
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

    pub fn make_move(&self, san: &str) -> Result<Game, IllegalMoveError> {
        let mut new = self.sans.clone();
        new.push(String::from(san));

        let mut possible_moves: HashMap<String, Move> = self.get_available_moves().into_iter()
            .map(|m| (m.generate_san(), m))
            .collect();
        println!("SAN: {}", san);
        println!("POSSIBLE MOVES!! ==> {:?}", possible_moves);

        if !possible_moves.contains_key(&san.to_string()) {
            Err(IllegalMoveError {})
        }
        else {
            let requested_move = possible_moves.remove(&san.to_string()).unwrap();
            let board = resolve_move(requested_move, self.board);
            Ok(Game { sans: new, board, is_first_player_turn: !self.is_first_player_turn })
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
