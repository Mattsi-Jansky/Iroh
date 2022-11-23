use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

pub struct InCheckHeuristic {}

impl Heuristic for InCheckHeuristic {
    fn evaluate(&self, state: &GameState, cache: &HeuristicsCache) -> i32 {
        if cache.is_check_first_player {-10}
        else if cache.is_check_second_player {10}
        else {0}
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::InCheck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_piece_in_check_returns_0() {
        let mut state = GameState::new();

        let cache = HeuristicsCache::from(&mut state);
        let result_enemy = InCheckHeuristic {}.evaluate(&state, &cache);

        assert_eq!(0, result_enemy);
    }

    #[test]
    fn given_second_player_in_check_returns_positive_10() {
        let mut state = GameState::from_fen("8/3k4/8/8/8/3R4/3K4/8 b - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = InCheckHeuristic {}.evaluate(&state, &cache);

        assert_eq!(10, result);
    }

    #[test]
    fn given_first_player_in_check_returns_negative_10() {
        let mut state = GameState::from_fen("8/3k4/3r4/8/8/8/3K4/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = InCheckHeuristic {}.evaluate(&state, &cache);

        assert_eq!(-10, result);
    }
}
