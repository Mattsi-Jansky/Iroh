use std::collections::HashMap;
use crate::heuristics::HeuristicType;

pub struct Weightings {
    weights: HashMap<HeuristicType,i32>
}

impl Weightings {
    pub fn new() -> Weightings {
        Weightings { weights: HashMap::new() }
    }

    pub fn push(mut self, heuristic_type: HeuristicType, weight: i32) -> Self {
        self.weights.insert(heuristic_type, weight);
        self
    }

    pub fn get(&self, heuristic_type: HeuristicType) -> Option<i32> {
        self.weights.get(&heuristic_type).map(|weight| weight.to_owned())
    }
}