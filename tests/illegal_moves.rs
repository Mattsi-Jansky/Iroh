use test_case::test_case;

use chess::*;

#[test]
fn cannot_make_illegal_move() {
    let game = ChessGame::new();

    let result = game.make_move("e7");

    assert!(result.is_err());
}
