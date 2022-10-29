use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

pub struct CheckHeuristic {}

impl Heuristic for CheckHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32 {
        if state.is_check(!is_first_player) {
            10
        } else {
            0
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Check
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_piece_in_check_returns_0() {
        let state = GameState::new();

        let result = CheckHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(0, result);
    }

    #[test]
    fn given_first_player_and_opponent_king_in_check_returns_10() {
        let state = GameState::from_fen("8/3k4/8/8/8/3R4/3K4/8 w - - 0 1");

        let result = CheckHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(10, result);
    }
}