use crate::game::GameInner;
use crate::serialisers::pgn::generate_pgn;

#[derive(Clone,PartialEq)]
pub enum Game {
    Ongoing {game: GameInner },
    IllegalMove,
    Draw{game: GameInner },
    Win{is_first_player_win: bool, sans: Vec<String>}
}

impl Game {
    pub fn unwrap(self) -> GameInner {
        match self {
            Game::Ongoing {game} => { game },
            _ => panic!("Game is not ongoing, cannot unwrap")
        }
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Game::IllegalMove)
    }

    pub fn generate_pgn(&self) -> Result<String,String> {
        match self {
            Game::Ongoing {game} | Game::Draw {game} => {
                Ok(generate_pgn(&game.sans, self))
            }
            Game::Win{is_first_player_win: _, sans} => {
                Ok(generate_pgn(sans, self))
            }
            Game::IllegalMove => { Err(String::from("An illegal move cannot generate a PGN")) }
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
}

#[cfg(test)]
mod tests {
    use super::Game::*;
    use super::*;

    #[test]
    fn given_ongoing_game_unwrap_should_return_game() {
        let game = GameInner::new().unwrap();
        let expected = game.clone();
        let result = Ongoing {game};

        let result = result.unwrap();

        assert_eq!(expected,result);
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_illegal_move_unwrap_should_panic() {
        let result = IllegalMove;

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_draw_unwrap_should_panic() {
        let game = GameInner::new().unwrap();
        let result = Draw {game};

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Game is not ongoing, cannot unwrap")]
    fn given_win_unwrap_should_panic() {
        let result = Win {is_first_player_win: true, sans: vec![]};

        result.unwrap();
    }

    #[test]
    fn given_illegal_move_generate_pgn_should_return_error() {
        let move_result = IllegalMove;

        let result = move_result.generate_pgn();

        assert!(result.is_err())
    }

    #[test]
    fn given_illegal_move_is_err_should_return_true() {
        let move_result = IllegalMove;

        assert!(move_result.is_err());
    }

    #[test]
    fn given_ongoing_game_is_err_should_return_false() {
        let game = GameInner::new().unwrap();
        let move_result = Ongoing {game};

        assert_eq!(false,move_result.is_err());
    }
}
