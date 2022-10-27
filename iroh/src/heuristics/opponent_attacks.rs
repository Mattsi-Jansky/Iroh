use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::GameState;

pub struct OpponentAttacksHeuristic {}

impl Heuristic for OpponentAttacksHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32 {
        0
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::OpponentAttacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_zero_opponent_attacks_at_starting_positions() {
        let state = GameState::new();

        let result = OpponentAttacksHeuristic {}.evaluate(&state, true);

        assert_eq!(0, result)
    }

    #[test]
    fn counts_number_of_attacks_two_pawns() {
        let state = GameState::from_fen("8/2PkP3/8/8/3K4/8/8/8 w - - 0 1");

        let result = OpponentAttacksHeuristic {}.evaluate(&state, true);

        assert_eq!(2, result)
    }
}