use crate::serialisers::pgn::generate_pgn;
use crate::state::captured_pieces::CapturedPieces;
use crate::state::GameState;

#[derive(Clone)]
pub enum Game {
    Ongoing { state: GameState },
    IllegalMove{ state: GameState },
    Draw{ state: GameState },
    Win{ is_first_player_win: bool, state: GameState }
}

impl Game {
    pub fn new() -> Game {
        Game::Ongoing { state: GameState::new() }
    }

    pub fn from_fen(fen: &str) -> Game {
        let game_state = GameState::from_fen(fen);
        game_state.determine_status()
    }

    pub fn unwrap_if_ongoing(self) -> GameState {
        match self {
            Game::Ongoing { state, .. } => { state }
            _ => panic!("Game is not ongoing, cannot unwrap")
        }
    }

    pub fn unwrap(&self) -> &GameState {
        match self {
            Game::Ongoing { state, .. } => { state }
            Game::IllegalMove { state } => { state }
            Game::Draw { state } => { state }
            Game::Win { state, .. } => { state }
        }
    }

    pub fn make_move(&self, san: &str) -> Game {
        match self {
            Game::Ongoing { state } => { state.make_move_san(san) }
            Game::IllegalMove { state } => { state.make_move_san(san) }
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

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Game::*;
    use super::*;

    #[test]
    fn given_ongoing_game_unwrap_should_return_game() {
        let game = Game::new().unwrap_if_ongoing();
        let expected = game.clone();
        let result = Ongoing { state: game };

        let result = result.unwrap_if_ongoing();

        assert_eq!(expected,result);
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_illegal_move_unwrap_should_panic() {
        let game = Game::new().unwrap_if_ongoing();
        let result = IllegalMove { state: game };

        result.unwrap_if_ongoing();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_draw_unwrap_should_panic() {
        let game = Game::new().unwrap_if_ongoing();
        let result = Draw { state: game };

        result.unwrap_if_ongoing();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_win_unwrap_should_panic() {
        let game = Game::new().unwrap_if_ongoing();
        let result = Win { is_first_player_win: true, state: game };

        result.unwrap_if_ongoing();
    }

    #[test]
    fn given_illegal_move_is_err_should_return_true() {
        let game = Game::new().unwrap_if_ongoing();
        let move_result = IllegalMove { state: game };

        assert!(move_result.is_err());
    }

    #[test]
    fn given_ongoing_game_is_err_should_return_false() {
        let game = Game::new().unwrap_if_ongoing();
        let move_result = Ongoing { state: game };

        assert_eq!(false,move_result.is_err());
    }
}
