use crate::heuristics::{Heuristic, HeuristicType};
use crate::heuristics::cache::HeuristicsCache;
use crate::moves::Move;
use crate::state::GameState;

pub struct CurrentPlayersAttacksHeuristic {}

impl Heuristic for CurrentPlayersAttacksHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32 {
        let result = evaluate_attacks(state, is_first_player, &state.possible_moves);
        if state.is_first_player_turn == is_first_player {
            result
        } else {
            -result
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Attacks
    }
}

pub struct OpponentPlayersAttacksHeuristic {}

impl Heuristic for OpponentPlayersAttacksHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32 {
        let result = evaluate_attacks(state, is_first_player, &heuristics_cache.opponents_possible_moves);
        if state.is_first_player_turn == is_first_player {
            -result
        } else {
            result
        }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::OpponentAttacks
    }
}

fn evaluate_attacks(state: &GameState, is_first_player: bool, possible_moves: &Vec<Move>) -> i32 {
    possible_moves.iter()
        .filter(|m| matches!(m, Move::AttackMove(..)))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use super::*;

    #[test]
    fn counts_number_of_attacks_zero() {
        let state = GameState::new();

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(0, result)
    }

    #[test]
    fn counts_number_of_attacks_two_pawns() {
        let state = GameState::from_fen("3k4/8/8/8/2pKp3/8/8/8 w - - 0 1");

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(2, result)
    }

    #[test]
    fn invert_if_evaluated_for_opposite_player() {
        let state = GameState::from_fen("3k4/8/8/8/2pKp3/8/8/8 w - - 0 1");

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_eq!(-2, result)
    }

    #[test]
    fn counts_zero_opponent_attacks_at_starting_positions() {
        let state = GameState::new();

        let result = OpponentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(0, result)
    }

    #[test]
    fn counts_opponents_number_of_attacks_two_pawns() {
        let state = GameState::from_fen("8/2PkP3/8/8/3K4/8/8/8 w - - 0 1");

        let result = OpponentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_eq!(-2, result)
    }

    #[test]
    fn given_evaluating_first_player_and_is_first_players_turn_current_players_attacks_positive_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 w - - 0 1");

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_that!(result > 0);
    }

    #[test]
    fn given_evaluating_first_player_and_is_second_players_turn_current_players_attacks_negative_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 b - - 0 1");

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_that!(result < 0);
    }

    #[test]
    fn given_evaluating_second_player_and_is_second_players_turn_current_players_attacks_positive_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 b - - 0 1");

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_that!(result > 0);
    }

    #[test]
    fn given_evaluating_second_player_and_is_first_players_turn_current_players_attacks_negative_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 w - - 0 1");

        let result = CurrentPlayersAttacksHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_that!(result < 0);
    }

    #[test]
    fn given_evaluating_first_player_and_is_first_players_turn_opponent_players_attacks_negative_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 w - - 0 1");

        let result = OpponentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_that!(result < 0);
    }

    #[test]
    fn given_evaluating_first_player_and_is_second_players_turn_opponent_players_attacks_positive_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 b - - 0 1");

        let result = OpponentPlayersAttacksHeuristic {}.evaluate(&state, true, &HeuristicsCache::from(&state, true));

        assert_that!(result > 0);
    }

    #[test]
    fn given_evaluating_second_player_and_is_second_players_turn_opponent_players_attacks_negative_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 b - - 0 1");

        let result = OpponentPlayersAttacksHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_that!(result < 0);
    }

    #[test]
    fn given_evaluating_second_player_and_is_first_players_turn_opponent_players_attacks_positive_value() {
        let state = GameState::from_fen("8/2PkP3/8/8/8/8/2pKp3/8 w - - 0 1");

        let result = OpponentPlayersAttacksHeuristic {}.evaluate(&state, false, &HeuristicsCache::from(&state, false));

        assert_that!(result > 0);
    }
}
