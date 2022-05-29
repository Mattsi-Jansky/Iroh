use std::fmt::Error;
use crate::game::Game;
use crate::serialisers::pgn::generate_pgn;

#[derive(Clone,PartialEq)]
pub enum MoveResult {
    OngoingGame{game: Game},
    IllegalMove,
    Draw{game: Game},
    Win{is_first_player_win: bool, sans: Vec<String>}
}

impl MoveResult {
    pub fn unwrap(self) -> Game {
        match self {
            MoveResult::OngoingGame{game} => { game },
            _ => panic!("Game is not ongoing, cannot unwrap")
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            MoveResult::IllegalMove => true,
            _ => false
        }
    }

    pub fn generate_pgn(&self) -> Result<String,String> {
        match self {
            MoveResult::OngoingGame {game} | MoveResult::Draw {game} => {
                Ok(generate_pgn(&game.sans, self))
            }
            MoveResult::Win{is_first_player_win, sans} => {
                Ok(generate_pgn(sans, self))
            }
            IllegalMove => { Err(String::from("An illegal move cannot generate a PGN")) }
        }
    }

    pub fn generate_fen(&self) -> Result<String,String> {
        match self {
            MoveResult::OngoingGame {game} | MoveResult::Draw {game} => {
                Ok(game.generate_fen())
            },
            _ => { Err(String::from("Cannot generate a FEN from an illegal move")) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MoveResult::*;
    use super::*;

    #[test]
    fn given_ongoing_game_unwrap_should_return_game() {
        let game = Game::new().unwrap();
        let expected = game.clone();
        let result = OngoingGame {game};

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
        let game = Game::new().unwrap();
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
        let game = Game::new().unwrap();
        let move_result = OngoingGame {game};

        assert_eq!(false,move_result.is_err());
    }
}
