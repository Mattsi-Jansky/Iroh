use crate::moves::move_generation::generate_moves;
use crate::moves::Move;
use crate::state::GameState;

pub struct HeuristicsCache {
    pub opponents_possible_moves: Vec<Move>,
    pub is_check_first_player: bool,
    pub is_check_second_player: bool,
    pub has_no_moves: bool,
}

impl HeuristicsCache {
    pub fn from(state: &mut GameState) -> HeuristicsCache {
        HeuristicsCache {
            opponents_possible_moves: generate_moves(state, !state.is_first_player_turn),
            is_check_first_player: state.is_check(true),
            is_check_second_player: state.is_check(false),
            has_no_moves: state.possible_moves.is_empty(),
        }
    }
}
