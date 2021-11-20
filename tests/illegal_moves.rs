use test_case::test_case;

use chess::game::Game;

#[test_case("e7")]
fn cannot_make_illegal_move(illegal_move: &str) {
    let game = Game::new();

    let result = game.make_move(illegal_move);

    assert!(result.is_err());
}
