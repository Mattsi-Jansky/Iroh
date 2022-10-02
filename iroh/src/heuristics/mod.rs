mod material;

use crate::state::GameState;

pub trait Heuristic {
    fn evaluate(&self, state: GameState, is_first_player: bool) -> u32;
}

pub struct Heuristics {
    heuristics: Vec<Box<dyn Heuristic>>
}

impl Default for Heuristics {
    fn default() -> Self {
        Heuristics { heuristics: vec![] }
    }
}
