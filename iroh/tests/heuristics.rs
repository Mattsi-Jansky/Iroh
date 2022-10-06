use galvanic_assert::assert_that;
use iroh::game::Game;
use iroh::heuristics::Heuristics;

#[test]
fn should_get_heuristic() {
    let game = Game::new();
    let heuristics = Heuristics::new();

    if let Game::Ongoing{ state, ..} = game {
        let value = heuristics.calculate(state);
        assert_that!(value > 0, otherwise "Unusual heuristic value")
    } else { panic!("Game should be ongoing") }
}
