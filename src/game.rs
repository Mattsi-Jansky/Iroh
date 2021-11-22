use std::collections::HashMap;
use crate::serialisers::pgn::generate_pgn;
use crate::state::board::Board;
use crate::state::captured_pieces::CapturedPieces;
use crate::error::IllegalMoveError;
use crate::moves::move_generation::generate_moves;
use crate::moves::{Move};
use crate::moves::resolve_move::resolve_move;
use crate::state::GameState;

pub struct Game {
    sans: Vec<String>,
    game_state: GameState
}

impl Game {
    pub fn new() -> Game {
        Game {
            sans: vec![],
            game_state: GameState::new()
        }
    }

    pub fn from_fen(fen: &str) -> Game {
        Game {
            sans: vec![],
            game_state: GameState::from_fen(fen),
        }
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        generate_moves(&self.game_state)
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
        let mut possible_moves: HashMap<String, Move> = self.get_available_moves().into_iter()
            .map(|m| (m.generate_san(), m))
            .collect();

        if let Some(requested_move) = possible_moves.remove(&san.to_string()) {
            let mut sans = self.sans.clone();
            sans.push(String::from(san));
            let game_state = resolve_move(requested_move, self.game_state.clone());
            Ok(Game {
                sans,
                game_state
            })
        } else {
            Err(IllegalMoveError {})
        }
    }

    pub fn get_captured_pieces(&self) -> &CapturedPieces {
        &self.game_state.captured_pieces
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
