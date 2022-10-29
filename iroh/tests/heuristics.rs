use galvanic_assert::assert_that;
use iroh::game::Game;
use iroh::heuristics::{Heuristics, HeuristicType};
use iroh::heuristics::attacks::{CurrentPlayersAttacksHeuristic, OpponentPlayersAttacksHeuristic};
use iroh::heuristics::material::MaterialHeuristic;
use iroh::heuristics::mobility::MobilityHeuristic;
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

    heuristics.push(MobilityHeuristic {});

    assert_eq!(20, heuristics.evaluate(&state, true))
}

#[test]
fn given_weighting_configuration_should_multiply_results_by_weights() {
    let state = GameState::new();
    let weightings = Weightings::new()
        .push(HeuristicType::Material,0.0)
        .push(HeuristicType::Mobility, 1.0);
    let weightings_double_move_weight = Weightings::new()
        .push(HeuristicType::Material,0.0)
        .push(HeuristicType::Mobility, 2.0);
    let mut heuristics = Heuristics::with_weighting(weightings);
    let mut heuristics_double_move_weight = Heuristics::with_weighting(weightings_double_move_weight);

    let moves_weighted_one = heuristics.evaluate(&state, true);
    let moves_weighted_two = heuristics_double_move_weight.evaluate(&state, true);

    assert_eq!(20, moves_weighted_one);
    assert_eq!(40, moves_weighted_two)
}

/*
This test is a bit confusing. There are two somewhat conflicting parameters: Who's turn it is, and
who we are evaluating for. In search we are evaluating for player `a`, we generate the result of a
move `a` can make, and evaluate the result of that move. But `b` moves after `a`. So we are
evaluating a game state in which it is `b`'s turn, because `a` just made a move.

As such `a` is the _opponent_ in that scenario. So, we want to maximise the _opponent_'s attacks.
So to reproduce this scenario we have a state in which it is the _second_ player's turn but we are
evaluating for the _first_ player.
 */
#[test]
fn more_opponents_attacks_are_better() {
    let state_hidden_queen = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    let state_developed_queen = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 1");

    let mut heuristics = Heuristics::blank();
    heuristics.push(CurrentPlayersAttacksHeuristic {});
    heuristics.push(OpponentPlayersAttacksHeuristic {});

    let result_hidden_queen = heuristics.evaluate(&state_hidden_queen, true);
    let result_developed_queen = heuristics.evaluate(&state_developed_queen, true);

    println!("Hidden {result_hidden_queen}, developed {result_developed_queen}");
    assert_that!(result_developed_queen > result_hidden_queen);
}
