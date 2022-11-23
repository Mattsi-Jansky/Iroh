use galvanic_assert::assert_that;
use iroh::heuristics::mobility::MobilityHeuristic;
use iroh::heuristics::weightings::Weightings;
use iroh::heuristics::{HeuristicType, Heuristics};
use iroh::state::GameState;

#[test]
fn given_starting_position_evaluates_to_zero() {
    let mut state = GameState::new();
    let heuristics = Heuristics::new();

    let result = heuristics.evaluate(&mut state);
    assert_eq!(0, result)
}

#[test]
fn given_winning_position_for_first_player_returns_positive_value() {
    let mut state =
        GameState::from_fen("rn1qk3/ppp2ppp/8/4Q3/1PB5/P1N2N2/2PPPPPP/R1B1R1K1 w q - 0 1");
    let heuristics = Heuristics::new();

    let value = heuristics.evaluate(&mut state);
    assert_that!(value > 0, otherwise "Heuristic should be positive when first player is winning")
}

#[test]
fn given_winning_position_for_second_player_returns_negative_value() {
    let mut state = GameState::from_fen("1k1r2nr/ppp2ppp/1nb1pb2/3pq3/8/8/PP5P/2BQK2R w K - 0 1");
    let heuristics = Heuristics::new();

    let value = heuristics.evaluate(&mut state);
    assert_that!(value < 0, otherwise "Heuristic should be negative when second player is winning")
}

#[test]
fn given_blank_heuristics_push_should_add_specified_heuristic() {
    let mut state =
        GameState::from_fen("rn1qk3/ppp2ppp/8/4Q3/1PB5/P1N2N2/2PPPPPP/R1B1R1K1 w q - 0 1");
    let mut heuristics = Heuristics::blank();

    assert_eq!(0, heuristics.evaluate(&mut state));

    heuristics.push(MobilityHeuristic {});

    assert_eq!(26, heuristics.evaluate(&mut state))
}

#[test]
fn given_weighting_configuration_should_multiply_results_by_weights() {
    let mut state = GameState::from_fen("4k3/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1");
    let weightings = Weightings::new()
        .push(HeuristicType::Material, 0.0)
        .push(HeuristicType::Mobility, 1.0);
    let weightings_double_move_weight = Weightings::new()
        .push(HeuristicType::Material, 0.0)
        .push(HeuristicType::Mobility, 2.0);
    let heuristics = Heuristics::with_weighting(weightings);
    let heuristics_double_move_weight = Heuristics::with_weighting(weightings_double_move_weight);

    let moves_weighted_one = heuristics.evaluate(&mut state);
    let moves_weighted_two = heuristics_double_move_weight.evaluate(&mut state);

    assert_eq!(15, moves_weighted_one);
    assert_eq!(30, moves_weighted_two)
}

//This test is based on a problem with heuristics discovered while debugging a failing search test,
//"discovered_check" in search.rs
#[test]
fn taking_rook_beats_returning_to_e5() {
    let mut better_state =
        GameState::from_fen("r4b1N/ppqk1ppp/2p5/3p4/3P4/8/PPPQ1PPP/R3R1K1 b - - 0 1");
    let mut worse_state =
        GameState::from_fen("r4b1r/ppqk1ppp/2p5/3pN3/3P4/8/PPPQ1PPP/R3R1K1 b - - 0 1");
    let heuristics = Heuristics::new();

    println!("Better value");
    let better_value = heuristics.evaluate(&mut better_state);
    println!("Worse value");
    let worse_value = heuristics.evaluate(&mut worse_state);

    assert_that!(better_value > worse_value, otherwise format!("Better position evaluates to {better_value}, but worse evaluated to {worse_value}"));
}
