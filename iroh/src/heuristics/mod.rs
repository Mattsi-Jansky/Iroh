mod material;

use std::rc::Rc;
use crate::heuristics::material::MaterialHeuristic;
use crate::state::GameState;

pub trait Heuristic {
    fn evaluate(&self, state: GameState, is_first_player: bool) -> u32;
}

pub struct Heuristics {
    heuristics: Rc<Vec<Box<dyn Heuristic>>>,
    pub value: u32
}

impl Default for Heuristics {
    fn default() -> Self {
        let mut heuristics: Vec<Box<dyn Heuristic>> = vec![];
        heuristics.push(Box::new(MaterialHeuristic {}));
        Heuristics { heuristics: Rc::from(heuristics), value: 0 }
    }
}

impl Clone for Heuristics {
    fn clone(&self) -> Self {
        Heuristics { heuristics: Rc::clone(&self.heuristics), value: self.value}
    }
}

impl Heuristics {
    fn calculate(&mut self, state: GameState) {
        todo!()
    }
}
