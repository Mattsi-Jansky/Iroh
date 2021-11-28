use test_case::test_case;

use chess::game::Game;

#[test_case("e7")]
#[test_case("ng3")]
#[test_case("bc2")]
#[test_case("qe3")]
#[test_case("banana")]
fn cannot_make_illegal_move(illegal_move: &str) {
    let game = Game::new();

    let result = game.make_move(illegal_move);

    assert!(result.is_err());
}

#[test]
fn cannot_move_off_board() {
    let game = Game::from_fen("8/8/8/8/8/8/8/3Q4 w - - 0 1");

    let result = game.make_move("qd9");
    let result2 = game.make_move("qd-1");

    assert!(result.is_err());
    assert!(result2.is_err());
}

#[test_case("8/8/2P5/3K4/8/8/8/8 w - - 0 1", "Kxc6")]
#[test_case("8/2P5/8/3N4/8/8/8/8 w - - 0 1", "Nxc7")]
#[test_case("8/2P5/8/4B3/8/8/8/8 w - - 0 1", "Bxc7")]
#[test_case("8/2P5/8/4Q3/8/8/8/8 w - - 0 1", "Qxc7")]
#[test_case("8/3P4/8/8/3R4/8/8/8 w - - 0 1", "Rxd7")]
#[test_case("8/8/8/2P1P3/3P4/8/8/8 w - - 0 1", "dxc5")]
#[test_case("8/8/8/2P1P3/3P4/8/8/8 w - - 0 1", "dxe5")]
fn cannot_take_friendly_piece(fen: &str, illegal_move: &str) {
    let game = Game::from_fen(fen);

    let result = game.make_move(illegal_move);

    assert!(result.is_err());
}

#[test]
fn pawn_cannot_capture_forwards() {
    let game = Game::from_fen("8/8/8/3p4/3P4/8/8/8 w - - 0 1");

    let result = game.make_move("d5");
    let result2 = game.make_move("dxd5");

    assert!(result.is_err());
    assert!(result2.is_err());
}