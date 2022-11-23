mod cache;
pub mod material;
pub mod mobility;
pub mod weightings;
pub mod attacks;
pub mod checks;
pub mod checkmates;

use cache::HeuristicsCache;
use crate::heuristics::attacks::AttacksHeuristic;
use crate::heuristics::checkmates::InCheckmateHeuristic;
use crate::heuristics::checks::InCheckHeuristic;
use crate::heuristics::HeuristicType::CheckMate;
use crate::heuristics::material::MaterialHeuristic;
use crate::heuristics::mobility::MobilityHeuristic;
use crate::heuristics::weightings::Weightings;
use crate::state::GameState;

#[derive(Hash,PartialEq,Eq,Debug)]
pub enum HeuristicType {
    Material,
    Mobility,
    Attacks,
    InCheck,
    CheckMate
}

pub trait Heuristic {
    fn evaluate(&self, state: &GameState, heuristics_cache: &HeuristicsCache) -> i32;
    fn get_type(&self) -> HeuristicType;
}

pub struct Heuristics {
    heuristics: Vec<Box<dyn Heuristic>>,
    weightings: Weightings
}

impl Default for Heuristics {
    fn default() -> Self {
        let heuristics: Vec<Box<dyn Heuristic>> = vec![
            Box::new(MaterialHeuristic {}),
            Box::new(MobilityHeuristic {}),
            Box::new(AttacksHeuristic {}),
            Box::new(InCheckHeuristic {}),
            Box::new(InCheckmateHeuristic {})
        ];
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
        Heuristics { weightings, ..Default::default() }
    }

    pub fn evaluate(&self, state: &mut GameState) -> i32 {
        let mut result = 0;
        let heuristics_cache = HeuristicsCache::from(state);
        for heuristic in self.heuristics.iter() {
            let heuristic_value = heuristic.evaluate(state, &heuristics_cache);
            let heuristic_weight = self.weightings.get(heuristic.get_type()).unwrap_or(1.0);
            let weighted_value = (heuristic_value as f32 * heuristic_weight).round() as i32;
            #[cfg(debug_assertions)]
            {
                let heuristic_type = heuristic.get_type();
                println!("Valued {heuristic_type:?} at {weighted_value}");
            }

            result += weighted_value;
        }
        result
    }

    pub fn push<T : Heuristic + 'static>(&mut self, heuristic: T) {
        self.heuristics.push(Box::new(heuristic));
    }
}
