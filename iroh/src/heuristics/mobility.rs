//I can't remember the correct name for this heuristic and don't have an internet
//connection, can't check wikipedia. It can be `mobility` for now.

use crate::state::GameState;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::heuristics::cache::HeuristicsCache;

pub struct MobilityHeuristic {}

impl Heuristic for MobilityHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32 {
        let moves = state.possible_moves.len() as i32;
        if state.is_first_player_turn == is_first_player { moves } else { -moves }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Mobility
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_number_of_moves_as_heuristic() {
      let state = GameState::new();

      let result = MobilityHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

      assert_eq!(20, result)
    }

    #[test]
    fn when_evaluating_position_for_player_not_making_move_returns_inverted_heuristic() {
        let state = GameState::new();

        let result = MobilityHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_eq!(-20, result)
    }
}
