//I can't remember the correct name for this heuristic and don't have an internet
//connection, can't check wikipedia. It can be `moves.rs` for now.

use crate::state::GameState;
use crate::heuristics::{Heuristic, HeuristicType};

pub struct MovesHeuristic {}

impl Heuristic for MovesHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32 {
        let moves = state.possible_moves.len() as i32;
        if state.is_first_player_turn == is_first_player { moves } else { -moves }
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_number_of_moves_as_heuristic() {
      let state = GameState::new();

      let result = MovesHeuristic{}.evaluate(&state, true);

      assert_eq!(20, result)
    }

    #[test]
    fn when_evaluating_position_for_player_not_making_move_returns_inverted_heuristic() {
        let state = GameState::new();

        let result = MovesHeuristic{}.evaluate(&state, false);

        assert_eq!(-20, result)
    }
}
