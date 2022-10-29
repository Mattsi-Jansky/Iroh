use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

pub struct OpponentInCheckHeuristic {}

impl Heuristic for OpponentInCheckHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, _: &HeuristicsCache) -> i32 {
        if state.is_check(!is_first_player) {
            10
        } else {
            0
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::EnemyInCheck
    }
}

pub struct SelfInCheckHeuristic {}

impl Heuristic for SelfInCheckHeuristic {

    fn evaluate(&self, state: &GameState, is_first_player: bool, _: &HeuristicsCache) -> i32 {
        if state.is_check(is_first_player) {
            -10
        } else {
            0
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::SelfInCheck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_piece_in_check_returns_0() {
        let state = GameState::new();

        let result_enemy = OpponentInCheckHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state));
        let result_self = SelfInCheckHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(0, result_enemy);
        assert_eq!(0, result_self);
    }

    #[test]
    fn given_first_player_and_opponent_king_in_check_returns_10() {
        let state = GameState::from_fen("8/3k4/8/8/8/3R4/3K4/8 b - - 0 1");

        let result = OpponentInCheckHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(10, result);
    }

    #[test]
    fn given_second_player_and_opponent_king_in_check_returns_10() {
        let state = GameState::from_fen("8/3k4/3r4/8/8/8/3K4/8 w - - 0 1");

        let result = OpponentInCheckHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state));

        assert_eq!(10, result);
    }

    #[test]
    fn given_first_player_and_self_in_check_self_check_returns_negative_10() {
        let state = GameState::from_fen("8/3k4/3r4/8/8/8/3K4/8 w - - 0 1");

        let result = SelfInCheckHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(-10, result);
    }
}
