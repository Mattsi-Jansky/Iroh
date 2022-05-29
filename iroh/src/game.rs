use std::collections::HashMap;
use crate::serialisers::pgn::generate_pgn;
use crate::state::captured_pieces::CapturedPieces;
use crate::error::IllegalMoveError;
use crate::move_result::MoveResult;
use crate::moves::move_generation::generate_moves;
use crate::moves::{Move};
use crate::moves::resolve_move::resolve_move;
use crate::serialisers::fen::generate_fen;
use crate::state::GameState;

#[derive(PartialEq,Debug,Clone)]
pub struct Game {
    pub(crate) sans: Vec<String>,
    game_state: GameState,
    possible_moves: Vec<Move>
}

impl Game {
    pub fn new() -> MoveResult {
        let state = GameState::new();
        let moves = generate_moves(&state);
        MoveResult::OngoingGame{game: Game {
            sans: vec![],
            game_state: state,
            possible_moves: moves
        }}
    }

    pub fn from_fen(fen: &str) -> MoveResult {
        let game_state = GameState::from_fen(fen);
        let possible_moves = generate_moves(&game_state);
        Self::determine_status(Game {
            sans: vec![],
            game_state,
            possible_moves
        })
    }

    pub fn generate_fen(&self) -> String {
        generate_fen(&self.game_state)
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        self.possible_moves.clone()
    }

    pub fn is_first_player_turn(&self) -> bool {
        self.game_state.is_first_player_turn
    }

    pub fn make_move(&self, san: &str) -> MoveResult {
        let possible_moves: HashMap<String, &Move> = self.possible_moves.iter()
            .map(|m| (m.generate_san(), m))
            .collect();

        if let Some(requested_move) = possible_moves.get(&san.to_string()) {
            let mut sans = self.sans.clone();
            sans.push(String::from(san));
            let game_state = resolve_move(requested_move, self.game_state.clone());
            let possible_moves = generate_moves(&game_state);
            Self::determine_status(Game {
                sans,
                game_state,
                possible_moves
            })
        } else {
            MoveResult::IllegalMove
        }
    }

    fn determine_status(game: Game) -> MoveResult {
        if game.possible_moves.is_empty() {
            if game.game_state.is_check(game.game_state.is_first_player_turn) {
                MoveResult::Win{ is_first_player_win: !game.is_first_player_turn(), sans: game.sans }
            }
            else { MoveResult::Draw{game} }
        } else { MoveResult::OngoingGame {game} }
    }

    pub fn get_captured_pieces(&self) -> &CapturedPieces {
        &self.game_state.captured_pieces
    }
}
