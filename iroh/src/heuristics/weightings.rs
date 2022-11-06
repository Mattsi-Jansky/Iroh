use std::collections::HashMap;
use crate::heuristics::HeuristicType;

pub struct Weightings {
    weights: HashMap<HeuristicType,f32>
}

impl Weightings {
    pub fn new() -> Weightings {
        Weightings { weights: HashMap::new() }
            .push(HeuristicType::Material, 11.0)
            .push(HeuristicType::Mobility, 0.25)
            .push(HeuristicType::Mobility, 0.5)
            .push(HeuristicType::InCheck, 2.0)
    }

    pub fn push(mut self, heuristic_type: HeuristicType, weight: f32) -> Self {
        self.weights.insert(heuristic_type, weight);
        self
    }

    pub fn get(&self, heuristic_type: HeuristicType) -> Option<f32> {
        self.weights.get(&heuristic_type).map(|weight| weight.to_owned())
    }
}

impl Default for Weightings {
    fn default() -> Self {
        Self::new()
    }
}
