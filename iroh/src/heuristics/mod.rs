mod material;
mod moves;

use std::rc::Rc;
use crate::heuristics::material::MaterialHeuristic;
use crate::heuristics::moves::MovesHeuristic;
use crate::state::GameState;

pub trait Heuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool) -> u32;
}

pub struct Heuristics {
    heuristics: Vec<Box<dyn Heuristic>>
}

impl Default for Heuristics {
    fn default() -> Self {
        let mut heuristics: Vec<Box<dyn Heuristic>> = vec![];
        heuristics.push(Box::new(MaterialHeuristic {}));
        heuristics.push(Box::new(MovesHeuristic {}));
        Heuristics { heuristics }
    }
}

impl Heuristics {
    pub fn new() -> Heuristics {
        Default::default()
    }

    pub fn evaluate(self, state: &GameState, is_first_player: bool) -> u32 {
        let mut result = 0;
        for heuristic in self.heuristics {
            result += heuristic.evaluate(state, is_first_player);
        }
        result
    }
}
