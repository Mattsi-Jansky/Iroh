pub mod material;
pub mod mobility;
pub mod weightings;

use crate::heuristics::material::MaterialHeuristic;
use crate::heuristics::mobility::MobilityHeuristic;
use crate::heuristics::weightings::Weightings;
use crate::state::GameState;

#[derive(Hash,PartialEq,Eq)]
pub enum HeuristicType {
    Material,
    Mobility
}

pub trait Heuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32;
    fn get_type(&self) -> HeuristicType;
}

pub struct Heuristics {
    heuristics: Vec<Box<dyn Heuristic>>,
    weightings: Weightings
}

impl Default for Heuristics {
    fn default() -> Self {
        let mut heuristics: Vec<Box<dyn Heuristic>> = vec![];
        heuristics.push(Box::new(MaterialHeuristic {}));
        heuristics.push(Box::new(MobilityHeuristic {}));
        Heuristics { heuristics, weightings: Weightings::new() }
    }
}

impl Heuristics {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn blank() -> Self {
        Heuristics { heuristics: vec![], weightings: Weightings::new() }
    }

    pub fn with_weighting(weightings: Weightings) -> Self {
        let mut result: Heuristics = Default::default();
        result.weightings = weightings;
        result
    }

    pub fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32 {
        let mut result = 0;
        for heuristic in self.heuristics.iter() {
            let heuristic_value = heuristic.evaluate(state, is_first_player);
            let heuristic_weight = self.weightings.get(heuristic.get_type()).unwrap_or(1.0);
            result += (heuristic_value as f32 * heuristic_weight).round() as i32;
        }
        result
    }

    pub fn push<T : Heuristic + 'static>(&mut self, heuristic: T) {
        self.heuristics.push(Box::new(heuristic));
    }
}
