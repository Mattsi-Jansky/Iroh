use crate::state::board::Board;
use crate::state::captured_pieces::CapturedPieces;

pub mod board;
pub mod piece;
pub mod coordinates;
pub mod captured_pieces;

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    turn_number: u16,
    pub board: Board,
    pub captured_pieces: CapturedPieces
}

impl GameState {
    pub fn new() -> GameState {
        GameState { turn_number: 1, board: Board::new(), captured_pieces: CapturedPieces::new() }
    }

    pub fn from_fen(fen: &str) -> GameState {
        GameState { turn_number: 1, board: Board::from_fen(fen), captured_pieces: CapturedPieces::new() }
    }

    pub fn is_first_player_turn(&self) -> bool {
        self.turn_number % 2 != 0
    }

    pub fn increment_turn_number(&mut self) {
        self.turn_number += 1;
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_numbered_turns_are_first_player_turns() {
        let game_state = GameState::new();

        assert!(game_state.is_first_player_turn());
    }

    #[test]
    fn even_numbered_turns_are_not_first_player_turns() {
        let mut game_state = GameState::new();

        game_state.increment_turn_number();

        assert_eq!(false, game_state.is_first_player_turn());
    }
}