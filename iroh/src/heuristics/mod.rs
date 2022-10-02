mod material;

use crate::state::GameState;

pub trait Heuristic {
    fn evaluate(&self, state: GameState, is_first_player: bool) -> u32;
}
