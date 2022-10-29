use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

const VERY_BIG_NUMBER: i32 = 999999999;

pub struct OpponentInCheckmateHeuristic {}

impl Heuristic for OpponentInCheckmateHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, cache: &HeuristicsCache) -> i32 {
        let not_my_turn = state.is_first_player_turn != is_first_player;
        let has_no_moves = cache.has_no_moves;
        let opponent_checked = is_first_player && cache.is_check_second_player
            || !is_first_player && cache.is_check_first_player;

        if not_my_turn && has_no_moves && opponent_checked {
            VERY_BIG_NUMBER
        } else {
            0
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::OpponentInCheckMate
    }
}

pub struct SelfInCheckmateHeuristic {}

impl Heuristic for SelfInCheckmateHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, cache: &HeuristicsCache) -> i32 {
        let is_my_turn = state.is_first_player_turn == is_first_player;
        let has_no_moves = cache.has_no_moves;
        let self_checked = is_first_player && cache.is_check_first_player
            || !is_first_player && cache.is_check_second_player;

        if is_my_turn && has_no_moves && self_checked {
            -VERY_BIG_NUMBER
        } else {
            0
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::SelfInCheckMate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_not_in_checkmate_value_0() {
        let state = GameState::new();

        let result_opponent = OpponentInCheckmateHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state));
        let result_self = SelfInCheckmateHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(0, result_opponent);
        assert_eq!(0, result_self);

    }

    #[test]
    fn given_opponent_in_checkmate_return_extremely_big_value() {
        let state = GameState::from_fen("R2k4/7R/8/8/8/8/8/3K4 b - - 0 1");

        let result = OpponentInCheckmateHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(VERY_BIG_NUMBER, result);
    }

    #[test]
    fn given_self_in_checkmate_return_negative_extremely_big_value() {
        let state = GameState::from_fen("R2k4/7R/8/8/8/8/8/3K4 b - - 0 1");

        let result = SelfInCheckmateHeuristic{}.evaluate(&state, false, &HeuristicsCache::from(&state));

        assert_eq!(-VERY_BIG_NUMBER, result);
    }
}
