use crate::heuristics::{Heuristic, HeuristicType};
use crate::moves::Move;
use crate::state::GameState;

pub struct AttacksHeuristic {}

impl Heuristic for AttacksHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32 {
        state.possible_moves.iter()
            .filter(|m| matches!(m, Move::AttackMove(..)))
            .count() as i32
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_number_of_attacks_zero() {
        let state = GameState::new();

        let result = AttacksHeuristic {}.evaluate(&state, false);

        assert_eq!(0, result)
    }

    #[test]
    fn counts_number_of_attacks_two_pawns() {
        let state = GameState::from_fen("3k4/8/8/8/2pKp3/8/8/8 w - - 0 1");

        let result = AttacksHeuristic {}.evaluate(&state, false);

        assert_eq!(2, result)
    }
}
