use std::collections::HashMap;
use crate::heuristics::HeuristicType;

pub struct Weightings {
    weights: HashMap<HeuristicType,f32>
}

impl Weightings {
    pub fn new() -> Weightings {
        let weightings = Weightings { weights: HashMap::new() };
        weightings.push(HeuristicType::Material, 2.0)
    }

    pub fn push(mut self, heuristic_type: HeuristicType, weight: f32) -> Self {
        self.weights.insert(heuristic_type, weight);
        self
    }

    pub fn get(&self, heuristic_type: HeuristicType) -> Option<f32> {
        self.weights.get(&heuristic_type).map(|weight| weight.to_owned())
    }
}
