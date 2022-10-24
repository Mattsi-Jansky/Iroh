use galvanic_assert::assert_that;
use iroh::game::Game;
use iroh::heuristics::{Heuristics, HeuristicType};
use iroh::heuristics::material::MaterialHeuristic;
use iroh::heuristics::weightings::Weightings;
use iroh::state::GameState;

#[test]
fn should_get_heuristic() {
    let state = GameState::new();
    let heuristics = Heuristics::new();

    let value = heuristics.evaluate(&state, true);
    assert_that!(value > 0, otherwise "Unusual heuristic value")
}

#[test]
fn given_blank_heuristics_push_should_add_specified_heuristic() {
    let state = GameState::new();
    let mut heuristics = Heuristics::blank();

    assert_eq!(0, heuristics.evaluate(&state, true));

    heuristics.push(MaterialHeuristic{});

    assert_eq!(39, heuristics.evaluate(&state, true))
}

#[test]
fn given_weighting_configuration_should_multiply_results_by_weights() {
    let state = GameState::new();
    let weightings = Weightings::new()
        .push(HeuristicType::Material,0)
        .push(HeuristicType::Moves, 1);
    let weightings_double_move_weight = Weightings::new()
        .push(HeuristicType::Material,0)
        .push(HeuristicType::Moves, 2);
    let mut heuristics = Heuristics::with_weighting(weightings);
    let mut heuristics_double_move_weight = Heuristics::with_weighting(weightings_double_move_weight);

    let moves_weighted_one = heuristics.evaluate(&state, true);
    let moves_weighted_two = heuristics_double_move_weight.evaluate(&state, true);

    assert_eq!(20, moves_weighted_one);
    assert_eq!(40, moves_weighted_two)
}