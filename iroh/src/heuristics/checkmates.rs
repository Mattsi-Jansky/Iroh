use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

const VERY_BIG_NUMBER: i32 = 999999999;

pub struct InCheckmateHeuristic {}

impl Heuristic for InCheckmateHeuristic {
    fn evaluate(&self, state: &GameState, cache: &HeuristicsCache) -> i32 {
        if cache.is_check_first_player && cache.has_no_moves {-VERY_BIG_NUMBER}
        else if cache.is_check_second_player && cache.has_no_moves {VERY_BIG_NUMBER}
        else {0}
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::CheckMate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_not_in_checkmate_value_0() {
        let state = GameState::new();

        let result = InCheckmateHeuristic {}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(0, result);
    }

    #[test]
    fn given_second_player_in_checkmate_return_extremely_big_positive_value() {
        let state = GameState::from_fen("R2k4/7R/8/8/8/8/8/3K4 b - - 0 1");

        let result = InCheckmateHeuristic {}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(VERY_BIG_NUMBER, result);
    }

    #[test]
    fn given_first_player_in_checkmate_return_extremely_big_negative_value() {
        let state = GameState::from_fen("r2K4/7r/8/8/8/8/8/3k4 w - - 0 1");

        let result = InCheckmateHeuristic {}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(-VERY_BIG_NUMBER, result);
    }
}
