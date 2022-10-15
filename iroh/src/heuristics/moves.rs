//I can't remember the correct name for this heuristic and don't have an internet
//connection, can't check wikipedia. It can be `moves.rs` for now.

use crate::state::GameState;
use crate::heuristics::{Heuristic, HeuristicType};

pub struct MovesHeuristic {}

impl Heuristic for MovesHeuristic {
  fn evaluate(&self, state: &GameState, is_first_player: bool) -> u32 {
    state.possible_moves.len() as u32
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
}
