use std::collections::HashMap;
use crate::serialisers::pgn::generate_pgn;
use crate::board::Board;
use crate::captured_pieces::CapturedPieces;
use crate::error::IllegalMoveError;
use crate::moves::move_generation::generate_moves;
use crate::moves::{Move};
use crate::moves::resolve_move::resolve_move;
use crate::piece::PieceType;

pub struct Game {
    sans: Vec<String>,
    board: Board,
    is_first_player_turn: bool,
    captured_pieces: CapturedPieces
}

impl Game {
    pub fn new() -> Game {
        Game {
            sans: vec![],
            board: Board::new(),
            is_first_player_turn: true,
            captured_pieces: CapturedPieces::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Game {
        Game {
            sans: vec![],
            board: Board::from_fen(fen),
            is_first_player_turn: true,
            captured_pieces: CapturedPieces::new(),
        }
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

        if let Some(requested_move) = possible_moves.remove(&san.to_string()) {
            let (board, captured_pieces) = resolve_move(requested_move, self.board, self.captured_pieces.clone(), self.is_first_player_turn);
            Ok(Game {
                sans: new,
                board,
                is_first_player_turn: !self.is_first_player_turn,
                captured_pieces
            })
        } else {
            Err(IllegalMoveError {})
        }
    }

    pub fn get_captured_pieces(&self) -> &CapturedPieces {
        &self.captured_pieces
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}