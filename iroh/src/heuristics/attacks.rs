use crate::heuristics::{Heuristic, HeuristicType};
use crate::heuristics::cache::HeuristicsCache;
use crate::moves::Move;
use crate::state::GameState;

pub struct AttacksHeuristic {}

impl Heuristic for AttacksHeuristic {
    fn evaluate(&self, state: &GameState, cache: &HeuristicsCache) -> i32 {
        let current_player_attacks = evaluate_attacks(&state.possible_moves);
        let opponent_attacks = evaluate_attacks(&cache.opponents_possible_moves);
        let first_player_attacks = if state.is_first_player_turn {current_player_attacks} else {opponent_attacks};
        let second_player_attacks = if state.is_first_player_turn {opponent_attacks} else {current_player_attacks};

        let mut result = 0;
        result += first_player_attacks;
        result -= second_player_attacks;
        result
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Attacks
    }
}

fn evaluate_attacks(possible_moves: &[Move]) -> i32 {
    possible_moves.iter()
        .filter(|m| matches!(m, Move::AttackMove(..)))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use super::*;

    #[test]
    fn given_neither_player_can_attack_returns_zero() {
        let mut state = GameState::new();

        let cache = HeuristicsCache::from(&mut state);
        let result = AttacksHeuristic {}.evaluate(&state, &cache);

        assert_eq!(0, result)
    }

    #[test]
    fn given_first_player_has_two_attacks_returns_positive_2() {
        let mut state = GameState::from_fen("3k4/8/8/8/2pKp3/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = AttacksHeuristic {}.evaluate(&state, &cache);

        assert_eq!(2, result)
    }

    #[test]
    fn given_second_player_has_two_attacks_returns_negative_2() {
        let mut state = GameState::from_fen("8/2PkP3/8/8/3K4/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = AttacksHeuristic {}.evaluate(&state, &cache);

        assert_eq!(-2, result)
    }

    #[test]
    fn given_equal_number_of_attacks_returns_0() {
        let mut state = GameState::from_fen("3k4/8/3q4/8/8/3Q4/8/3K4 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = AttacksHeuristic {}.evaluate(&state, &cache);

        assert_eq!(0, result)
    }
}
