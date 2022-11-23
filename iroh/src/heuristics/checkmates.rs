use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

const VERY_BIG_NUMBER: i32 = 999999999;

pub struct InCheckmateHeuristic {}

impl Heuristic for InCheckmateHeuristic {
    fn evaluate(&self, state: &GameState, cache: &HeuristicsCache) -> i32 {
        if cache.is_check_first_player && cache.has_no_moves {
            -VERY_BIG_NUMBER + (state.turn_number * 10) as i32
        } else if cache.is_check_second_player && cache.has_no_moves {
            VERY_BIG_NUMBER - (state.turn_number * 10) as i32
        } else {
            0
        }
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
        let mut state = GameState::new();

        let cache = HeuristicsCache::from(&mut state);
        let result = InCheckmateHeuristic {}.evaluate(&state, &cache);

        assert_eq!(0, result);
    }

    #[test]
    fn given_second_player_in_checkmate_return_extremely_big_positive_value() {
        let mut state = GameState::from_fen("R2k4/7R/8/8/8/8/8/3K4 b - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = InCheckmateHeuristic {}.evaluate(&state, &cache);

        assert_eq!(VERY_BIG_NUMBER - 10, result);
    }

    #[test]
    fn given_first_player_in_checkmate_return_extremely_big_negative_value() {
        let mut state = GameState::from_fen("r2K4/7r/8/8/8/8/8/3k4 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = InCheckmateHeuristic {}.evaluate(&state, &cache);

        assert_eq!(-VERY_BIG_NUMBER + 10, result);
    }

    #[test]
    fn given_second_player_in_checkmate_should_reduce_heuristic_by_number_of_turns_taken() {
        let mut state_one_turn = GameState::from_fen("R2k4/7R/8/8/8/8/8/3K4 b - - 0 1");
        state_one_turn.turn_number = 1;
        let mut state_ten_turns = GameState::from_fen("R2k4/7R/8/8/8/8/8/3K4 b - - 0 1");
        state_ten_turns.turn_number = 10;

        let mut cache = HeuristicsCache::from(&mut state_one_turn);
        let result_one_turn = InCheckmateHeuristic {}.evaluate(&mut state_one_turn, &cache);
        cache = HeuristicsCache::from(&mut state_ten_turns);
        let result_ten_turns = InCheckmateHeuristic {}.evaluate(&mut state_ten_turns, &cache);

        assert_eq!(VERY_BIG_NUMBER - 10, result_one_turn);
        assert_eq!(VERY_BIG_NUMBER - 100, result_ten_turns);
    }

    #[test]
    fn given_first_player_in_checkmate_should_reduce_heuristic_by_number_of_turns_taken() {
        let mut state_one_turn = GameState::from_fen("r2K4/7r/8/8/8/8/8/3k4 w - - 0 1");
        state_one_turn.turn_number = 1;
        let mut state_ten_turns = GameState::from_fen("r2K4/7r/8/8/8/8/8/3k4 w - - 0 1");
        state_ten_turns.turn_number = 10;

        let mut cache = HeuristicsCache::from(&mut state_one_turn);
        let result_one_turn = InCheckmateHeuristic {}.evaluate(&mut state_one_turn, &cache);
        cache = HeuristicsCache::from(&mut state_ten_turns);
        let result_ten_turns = InCheckmateHeuristic {}.evaluate(&mut state_ten_turns, &cache);

        assert_eq!(-VERY_BIG_NUMBER + 10, result_one_turn);
        assert_eq!(-VERY_BIG_NUMBER + 100, result_ten_turns);
    }
}
