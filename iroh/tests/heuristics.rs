use galvanic_assert::assert_that;
use iroh::game::Game;

#[test]
fn should_get_heuristic() {
    let game = Game::new();

    if let Game::Ongoing{ heuristics, ..} = game {
        assert_that!(heuristics.value > 0, otherwise "Unusual heuristic value")
    } else { panic!("Game should be ongoing") }
}
