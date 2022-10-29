use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

pub struct EnemyInCheckHeuristic {}

impl Heuristic for EnemyInCheckHeuristic {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_piece_in_check_returns_0() {
        let state = GameState::new();

        let result = EnemyInCheckHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(0, result);
    }

    #[test]
    fn given_first_player_and_opponent_king_in_check_returns_10() {
        let state = GameState::from_fen("8/3k4/8/8/8/3R4/3K4/8 b - - 0 1");

        let result = EnemyInCheckHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state));

        assert_eq!(10, result);
    }

    #[test]
    fn given_second_player_and_opponent_king_in_check_returns_10() {
        let state = GameState::from_fen("8/3k4/3r4/8/8/8/3K4/8 w - - 0 1");

        let result = EnemyInCheckHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state));

        assert_eq!(10, result);
    }

    // #[test]
    // fn given_first_player_and_self_in_check_returns_negative_10() {
    //     let state = GameState::from_fen("8/3k4/3r4/8/8/8/3K4/8 w - - 0 1");
    //
    //     let result = CheckHeuristic{}.evaluate(&state, true, &HeuristicsCache::from(&state, true));
    //
    //     assert_eq!(-10, result);
    // }
}
