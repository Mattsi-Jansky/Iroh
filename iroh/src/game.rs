use std::collections::HashMap;
use crate::serialisers::pgn::generate_pgn;
use crate::state::captured_pieces::CapturedPieces;
use crate::error::IllegalMoveError;
use crate::moves::move_generation::generate_moves;
use crate::moves::{Move};
use crate::moves::resolve_move::resolve_move;
use crate::serialisers::fen::generate_fen;
use crate::state::GameState;

pub struct Game {
    sans: Vec<String>,
    game_state: GameState,
    possible_moves: Vec<Move>
}

impl Game {
    pub fn new() -> Game {
        let state = GameState::new();
        let moves = generate_moves(&state);
        Game {
            sans: vec![],
            game_state: state,
            possible_moves: moves
        }
    }

    pub fn from_fen(fen: &str) -> Game {
        let state = GameState::from_fen(fen);
        let moves = generate_moves(&state);
        Game {
            sans: vec![],
            game_state: state,
            possible_moves: moves
        }
    }

    pub fn generate_fen(&self) -> String {
        generate_fen(&self.game_state)
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        self.possible_moves.clone()
    }

    pub fn get_pgn(&self) -> String {
        if self.sans.is_empty() {
            String::new()
        }
        else {
            generate_pgn(&self.sans, self.is_game_ongoing(), self.game_state.is_first_player_turn)
        }
    }

    pub fn is_game_ongoing(&self) -> bool {
        !self.possible_moves.is_empty()
    }

    pub fn is_first_player_turn(&self) -> bool {
        self.game_state.is_first_player_turn
    }

    pub fn make_move(&self, san: &str) -> Result<Game, IllegalMoveError> {
        let possible_moves: HashMap<String, &Move> = self.possible_moves.iter()
            .map(|m| (m.generate_san(), m))
            .collect();

        if let Some(requested_move) = possible_moves.get(&san.to_string()) {
            let mut sans = self.sans.clone();
            sans.push(String::from(san));
            let game_state = resolve_move(&requested_move, self.game_state.clone());
            let moves = generate_moves(&game_state);
            Ok(Game {
                sans,
                game_state,
                possible_moves: moves
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
