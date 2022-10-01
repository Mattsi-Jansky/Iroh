use crate::serialisers::pgn::generate_pgn;
use crate::state::captured_pieces::CapturedPieces;
use crate::state::GameState;

#[derive(Clone,PartialEq,Eq)]
pub enum Game {
    Ongoing { game: GameState },
    IllegalMove{ game: GameState },
    Draw{ game: GameState },
    Win{ is_first_player_win: bool, state: GameState }
}

impl Game {
    pub fn new() -> Game {
        Game::Ongoing {game: GameState::new()}
    }

    pub fn from_fen(fen: &str) -> Game {
        let game_state = GameState::from_fen(fen);
        game_state.determine_status()
    }

    pub fn unwrap(self) -> GameState {
        match self {
            Game::Ongoing {game} => { game },
            _ => panic!("Game is not ongoing, cannot unwrap")
        }
    }

    pub fn make_move(&self, san: &str) -> Game {
        match self {
            Game::Ongoing { game } => { game.make_move(san) }
            Game::IllegalMove { game } => { game.make_move(san) }
            Game::Draw { .. } | Game::Win { .. } => { panic!("Cannot make move on a finished game") }
        }
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Game::IllegalMove {..})
    }

    pub fn generate_pgn(&self) -> Result<String,String> {
        match self {
            Game::Ongoing {game} | Game::Draw {game} => {
                Ok(generate_pgn(&game.sans, self))
            }
            Game::Win{ state: game, ..} => {
                Ok(generate_pgn(&game.sans, self))
            }
            Game::IllegalMove {..} => { Err(String::from("An illegal move cannot generate a PGN")) }
        }
    }

    pub fn generate_fen(&self) -> Result<String,String> {
        match self {
            Game::Ongoing {game} | Game::Draw {game} => {
                Ok(game.generate_fen())
            },
            _ => { Err(String::from("Cannot generate a FEN from an illegal move")) }
        }
    }

    pub fn captured_pieces(&self) -> Result<&CapturedPieces,String> {
        match self {
            Game::Ongoing {game} | Game::Draw{game} => {
                Ok(game.captured_pieces())
            },
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
        let result = Ongoing {game};

        let result = result.unwrap();

        assert_eq!(expected,result);
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_illegal_move_unwrap_should_panic() {
        let game = Game::new().unwrap();
        let result = IllegalMove { game };

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_draw_unwrap_should_panic() {
        let game = Game::new().unwrap();
        let result = Draw {game};

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
        let move_result = IllegalMove { game };

        assert!(move_result.is_err());
    }

    #[test]
    fn given_ongoing_game_is_err_should_return_false() {
        let game = Game::new().unwrap();
        let move_result = Ongoing {game};

        assert_eq!(false,move_result.is_err());
    }
}
