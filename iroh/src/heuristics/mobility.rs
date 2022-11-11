//I can't remember the correct name for this heuristic and don't have an internet
//connection, can't check wikipedia. It can be `mobility` for now.

use crate::state::GameState;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::heuristics::cache::HeuristicsCache;

pub struct MobilityHeuristic {}

impl Heuristic for MobilityHeuristic {
    fn evaluate(&self, state: &GameState, cache: &HeuristicsCache) -> i32 {
        let current_player_moves = state.possible_moves.len() as i32;
        let opposing_player_moves = cache.opponents_possible_moves.len() as i32;
        let first_player_moves = if state.is_first_player_turn {current_player_moves} else {opposing_player_moves};
        let second_player_moves = if state.is_first_player_turn {opposing_player_moves} else {current_player_moves};

        let mut result = 0;
        result += first_player_moves;
        result -= second_player_moves;
        result
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

      let result = MobilityHeuristic {}.evaluate(&state, &HeuristicsCache::from(&state));

      assert_eq!(0, result)
    }

    #[test]
    fn given_first_player_is_more_developed_result_is_positive() {
        let state = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/1NBQ1BN1/PPPPPPPP/R3K2R w KQkq - 0 1");

        let result = MobilityHeuristic {}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(27, result)
    }

    #[test]
    fn given_second_player_is_more_developed_result_is_negative() {
        let state = GameState::from_fen("r3k2r/pppppppp/1nbq1bn1/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let result = MobilityHeuristic {}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(-27, result)
    }
}
