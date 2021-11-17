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

    pub fn make_move(self, san: &str) -> Result<Game, IllegalMoveError> {
        let mut new = self.sans.clone();
        new.push(String::from(san));

        let requested_move = parse_san(san);
        let possible_moves = self.get_available_moves();

        if !possible_moves.contains(&requested_move) {
            println!("Cannot get move {:?} from {:?}", requested_move, possible_moves);
            Err(IllegalMoveError {})
        }
        else {
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
