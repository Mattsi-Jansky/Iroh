mod cache;
pub mod material;
pub mod mobility;
pub mod weightings;
pub mod attacks;
pub mod checks;

use cache::HeuristicsCache;
use crate::heuristics::attacks::{CurrentPlayersAttacksHeuristic, OpponentPlayersAttacksHeuristic};
use crate::heuristics::checks::{EnemyInCheckHeuristic, SelfInCheckHeuristic};
use crate::heuristics::material::MaterialHeuristic;
use crate::heuristics::mobility::MobilityHeuristic;
use crate::heuristics::weightings::Weightings;
use crate::state::GameState;

#[derive(Hash,PartialEq,Eq,Debug)]
pub enum HeuristicType {
    Material,
    Mobility,
    Attacks,
    OpponentAttacks,
    EnemyInCheck,
    SelfInCheck
}

pub trait Heuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool, heuristics_cache: &HeuristicsCache) -> i32;
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
            Box::new(CurrentPlayersAttacksHeuristic {}),
            Box::new(OpponentPlayersAttacksHeuristic {}),
            Box::new(EnemyInCheckHeuristic {}),
            Box::new(SelfInCheckHeuristic {}),
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

    pub fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32 {
        let mut result = 0;
        let heuristics_cache = HeuristicsCache::from(state);
        for heuristic in self.heuristics.iter() {
            let heuristic_value = heuristic.evaluate(state, is_first_player, &heuristics_cache);
            let heuristic_weight = self.weightings.get(heuristic.get_type()).unwrap_or(1.0);

            result += (heuristic_value as f32 * heuristic_weight).round() as i32;
        }
        result
    }

    pub fn push<T : Heuristic + 'static>(&mut self, heuristic: T) {
        self.heuristics.push(Box::new(heuristic));
    }
}
