mod material;

use std::rc::Rc;
use crate::heuristics::material::MaterialHeuristic;
use crate::state::GameState;

pub trait Heuristic {
    fn evaluate(&self, state: GameState, is_first_player: bool) -> u32;
}

pub struct Heuristics {
    heuristics: Vec<Box<dyn Heuristic>>
}

impl Default for Heuristics {
    fn default() -> Self {
        let mut heuristics: Vec<Box<dyn Heuristic>> = vec![];
        heuristics.push(Box::new(MaterialHeuristic {}));
        Heuristics { heuristics}
    }
}

impl Heuristics {
    pub fn new() -> Heuristics {
        Default::default()
    }

    pub fn calculate(self, state: GameState) -> u32 {
        todo!()
    }
}
