use galvanic_assert::assert_that;
use iroh::game::Game;
use iroh::heuristics::Heuristics;
use iroh::heuristics::material::MaterialHeuristic;
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
