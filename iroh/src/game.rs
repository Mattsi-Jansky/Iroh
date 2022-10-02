use std::rc::Rc;
use crate::heuristics::{Heuristics};
use crate::serialisers::pgn::generate_pgn;
use crate::state::captured_pieces::CapturedPieces;
use crate::state::GameState;

#[derive(Clone)]
pub enum Game {
    Ongoing { state: GameState, heuristics: Rc<Heuristics> },
    IllegalMove{ state: GameState, heuristics: Rc<Heuristics> },
    Draw{ state: GameState },
    Win{ is_first_player_win: bool, state: GameState }
}

impl Game {
    pub fn new() -> Game {
        Game::Ongoing { state: GameState::new(), heuristics: Default::default() }
    }

    pub fn from_fen(fen: &str) -> Game {
        let game_state = GameState::from_fen(fen);
        game_state.determine_status(&Rc::new(Default::default()))
    }

    pub fn unwrap(self) -> GameState {
        match self {
            Game::Ongoing { state, .. } => { state }
            _ => panic!("Game is not ongoing, cannot unwrap")
        }
    }

    pub fn make_move(&self, san: &str) -> Game {
        match self {
            Game::Ongoing { state, heuristics } => { state.make_move(san, Rc::clone(heuristics)) }
            Game::IllegalMove { state, heuristics } => { state.make_move(san, Rc::clone(heuristics)) }
            Game::Draw { .. } | Game::Win { .. } => { panic!("Cannot make move on a finished game") }
        }
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Game::IllegalMove {..})
    }

    pub fn generate_pgn(&self) -> Result<String,String> {
        match self {
            Game::Ongoing { state, .. } | Game::Draw { state } => {
                Ok(generate_pgn(&state.sans, self))
            }
            Game::Win{ state, ..} => {
                Ok(generate_pgn(&state.sans, self))
            }
            Game::IllegalMove {..} => { Err(String::from("An illegal move cannot generate a PGN")) }
        }
    }

    pub fn generate_fen(&self) -> Result<String,String> {
        match self {
            Game::Ongoing { state, .. } | Game::Draw { state } => {
                Ok(state.generate_fen())
            },
            _ => { Err(String::from("Cannot generate a FEN from an illegal move")) }
        }
    }

    pub fn captured_pieces(&self) -> Result<&CapturedPieces,String> {
        match self {
            Game::Ongoing { state, .. } | Game::Draw{ state } => {
                Ok(state.captured_pieces())
            }
            _ => { Err(String::from("Cannot get captured pieces from illegal move")) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game::*;
    use super::*;

    #[test]
    fn given_ongoing_game_unwrap_should_return_game() {
        let game = Game::new().unwrap();
        let expected = game.clone();
        let result = Ongoing { state: game, heuristics: Default::default() };

        let result = result.unwrap();

        assert_eq!(expected,result);
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_illegal_move_unwrap_should_panic() {
        let game = Game::new().unwrap();
        let result = IllegalMove { state: game, heuristics: Default::default() };

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_draw_unwrap_should_panic() {
        let game = Game::new().unwrap();
        let result = Draw { state: game };

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_win_unwrap_should_panic() {
        let game = Game::new().unwrap();
        let result = Win { is_first_player_win: true, state: game };

        result.unwrap();
    }

    #[test]
    fn given_illegal_move_is_err_should_return_true() {
        let game = Game::new().unwrap();
        let move_result = IllegalMove { state: game, heuristics: Default::default() };

        assert!(move_result.is_err());
    }

    #[test]
    fn given_ongoing_game_is_err_should_return_false() {
        let game = Game::new().unwrap();
        let move_result = Ongoing { state: game, heuristics: Default::default() };

        assert_eq!(false,move_result.is_err());
    }
}
