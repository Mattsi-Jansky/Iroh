use crate::moves::Move;
use crate::moves::move_generation::generate_moves;
use crate::state::GameState;

pub struct HeuristicsCache {
    pub opponents_possible_moves: Vec<Move>
}

impl HeuristicsCache {
    pub fn from(state: &GameState) -> HeuristicsCache {
        HeuristicsCache{
            opponents_possible_moves: generate_moves(state, !state.is_first_player_turn)
        }
    }
}
