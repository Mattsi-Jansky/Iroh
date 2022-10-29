use crate::heuristics::{Heuristic, HeuristicType};
use crate::heuristics::cache::HeuristicsCache;
use crate::moves::Move;
use crate::state::GameState;

pub struct AttacksHeuristic {}

impl Heuristic for AttacksHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32 {
        evaluate_attacks(state, is_first_player, &state.possible_moves)
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Attacks
    }
}

pub struct OpponentAttacksHeuristic {}

impl Heuristic for OpponentAttacksHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32 {
        evaluate_attacks(state, is_first_player, &heuristics_cache.opponents_possible_moves)
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::OpponentAttacks
    }
}

fn evaluate_attacks(state: &GameState, is_first_player: bool, possible_moves: &Vec<Move>) -> i32 {
    let result = possible_moves.iter()
        .filter(|m| matches!(m, Move::AttackMove(..)))
        .count() as i32;
    if is_first_player == state.is_first_player_turn {
        result
    } else {
        -result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_number_of_attacks_zero() {
        let state = GameState::new();

        let result = AttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(0, result)
    }

    #[test]
    fn counts_number_of_attacks_two_pawns() {
        let state = GameState::from_fen("3k4/8/8/8/2pKp3/8/8/8 w - - 0 1");

        let result = AttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(2, result)
    }

    #[test]
    fn invert_if_evaluated_for_opposite_player() {
        let state = GameState::from_fen("3k4/8/8/8/2pKp3/8/8/8 w - - 0 1");

        let result = AttacksHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_eq!(-2, result)
    }

    #[test]
    fn counts_zero_opponent_attacks_at_starting_positions() {
        let state = GameState::new();

        let result = OpponentAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(0, result)
    }

    #[test]
    fn counts_opponents_number_of_attacks_two_pawns() {
        let state = GameState::from_fen("8/2PkP3/8/8/3K4/8/8/8 w - - 0 1");

        let result = OpponentAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(2, result)
    }
}
