use iroh::evaluations::evaluate;
use iroh::state::GameState;

#[test]
fn simple_puzzle() {
    let state = GameState::from_fen("3k4/8/8/3p4/4P3/8/8/3K4 w - - 0 1");

    let result = evaluate(&state);

    assert_eq!(String::from("exd5"), result.best_move);
}
