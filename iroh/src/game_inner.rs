use std::collections::HashMap;
use crate::game::Game;
use crate::state::captured_pieces::CapturedPieces;
use crate::moves::move_generation::generate_moves;
use crate::moves::{Move};
use crate::moves::resolve_move::resolve_move;
use crate::serialisers::fen::generate_fen;
use crate::state::GameState;

#[derive(PartialEq, Eq, Debug,Clone)]
pub struct GameInner {
    pub(crate) sans: Vec<String>,
    game_state: GameState,
    possible_moves: Vec<Move>
}

impl GameInner {
    pub(crate) fn new() -> GameInner {
        let state = GameState::new();
        let moves = generate_moves(&state);
        GameInner {
            sans: vec![],
            game_state: state,
            possible_moves: moves
        }
    }

    pub fn from_fen(fen: &str) -> Game {
        let game_state = GameState::from_fen(fen);
        let possible_moves = generate_moves(&game_state);
        Self::determine_status(GameInner {
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

    pub fn make_move(&self, san: &str) -> Game {
        let possible_moves: HashMap<String, &Move> = self.possible_moves.iter()
            .map(|m| (m.generate_san(), m))
            .collect();

        if let Some(requested_move) = possible_moves.get(&san.to_string()) {
            let mut sans = self.sans.clone();
            sans.push(String::from(san));
            let game_state = resolve_move(requested_move, self.game_state.clone());
            let possible_moves = generate_moves(&game_state);
            Self::determine_status(GameInner {
                sans,
                game_state,
                possible_moves
            })
        } else {
            Game::IllegalMove { game: self.clone() }
        }
    }

    fn determine_status(game: GameInner) -> Game {
        if game.possible_moves.is_empty() {
            if game.game_state.is_check(game.game_state.is_first_player_turn) {
                Game::Win{ is_first_player_win: !game.is_first_player_turn(), game }
            }
            else { Game::Draw{game} }
        } else { Game::Ongoing {game} }
    }

    pub fn get_captured_pieces(&self) -> &CapturedPieces {
        &self.game_state.captured_pieces
    }
}
