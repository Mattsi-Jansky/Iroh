use galvanic_assert::assert_that;
use iroh::game::Game;
use iroh::heuristics::{Heuristics, HeuristicType};
use iroh::heuristics::attacks::AttacksHeuristic;
use iroh::heuristics::material::MaterialHeuristic;
use iroh::heuristics::mobility::MobilityHeuristic;
use iroh::heuristics::weightings::Weightings;
use iroh::state::GameState;

#[test]
fn given_starting_position_evaluates_to_zero() {
    let state = GameState::new();
    let heuristics = Heuristics::new();

    let result = heuristics.evaluate(&state);
    assert_eq!(0, result)
}

#[test]
fn given_winning_position_for_first_player_returns_positive_value() {
    let state = GameState::from_fen("rn1qk3/ppp2ppp/8/4Q3/1PB5/P1N2N2/2PPPPPP/R1B1R1K1 w q - 0 1");
    let heuristics = Heuristics::new();

    let value = heuristics.evaluate(&state);
    assert_that!(value > 0, otherwise "Heuristic should be positive when first player is winning")
}

#[test]
fn given_winning_position_for_second_player_returns_negative_value() {
    let state = GameState::from_fen("1k1r2nr/ppp2ppp/1nb1pb2/3pq3/8/8/PP5P/2BQK2R w K - 0 1");
    let heuristics = Heuristics::new();

    let value = heuristics.evaluate(&state);
    assert_that!(value < 0, otherwise "Heuristic should be negative when second player is winning")
}

#[test]
fn given_blank_heuristics_push_should_add_specified_heuristic() {
    let state = GameState::from_fen("rn1qk3/ppp2ppp/8/4Q3/1PB5/P1N2N2/2PPPPPP/R1B1R1K1 w q - 0 1");
    let mut heuristics = Heuristics::blank();

    assert_eq!(0, heuristics.evaluate(&state));

    heuristics.push(MobilityHeuristic {});

    assert_eq!(13, heuristics.evaluate(&state))
}

#[test]
fn given_weighting_configuration_should_multiply_results_by_weights() {
    let state = GameState::from_fen("4k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1");
    let weightings = Weightings::new()
        .push(HeuristicType::Material,0.0)
        .push(HeuristicType::Mobility, 1.0);
    let weightings_double_move_weight = Weightings::new()
        .push(HeuristicType::Material,0.0)
        .push(HeuristicType::Mobility, 2.0);
    let mut heuristics = Heuristics::with_weighting(weightings);
    let mut heuristics_double_move_weight = Heuristics::with_weighting(weightings_double_move_weight);

    let moves_weighted_one = heuristics.evaluate(&state);
    let moves_weighted_two = heuristics_double_move_weight.evaluate(&state);

    assert_eq!(15, moves_weighted_one);
    assert_eq!(30, moves_weighted_two)
}
//
// #[test]
// fn wat() {
//     let state = GameState::from_fen("8/k7/3q4/5p2/6P1/8/K7/1Q6 b - - 1 1");
//     let heuristics = Heuristics::new();
//
//     let value = heuristics.evaluate(&state);
//
//     assert_eq!(5,value)
// }
