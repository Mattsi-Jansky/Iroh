#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::collection::*;
use test_case::test_case;

use chess::state::piece::PieceType;
use chess::game::Game;

#[test]
fn new_game_has_blank_pgn() {
    let game = Game::new();

    let result = game.get_pgn();

    assert_eq!(result,"");
}

#[test_case("a3")]
#[test_case("b3")]
#[test_case("c3")]
#[test_case("d3")]
#[test_case("e3")]
#[test_case("f3")]
#[test_case("g3")]
#[test_case("h3")]
fn first_pawn_move(san: &str) {
    let mut game = Game::new();

    game = game.make_move(san).unwrap();
    let result = game.get_pgn();

    assert_eq!(format!("1. {} *", san), result);
}

#[test_case("a5")]
#[test_case("b5")]
#[test_case("c5")]
#[test_case("d5")]
#[test_case("e5")]
#[test_case("f5")]
#[test_case("g5")]
#[test_case("h5")]
#[test_case("a6")]
#[test_case("b6")]
#[test_case("c6")]
#[test_case("d6")]
#[test_case("e6")]
#[test_case("f6")]
#[test_case("g6")]
#[test_case("h6")]
fn second_pawn_move(san: &str) {
    let mut game = Game::new();

    game = game.make_move("e4").unwrap();
    game = game.make_move(san).unwrap();
    let result = game.get_pgn();

    assert_eq!(format!("1. e4 {} *", san), result);
}

#[test]
fn second_turn() {
    let mut game = Game::new();

    game = game.make_move("e4").unwrap();
    game = game.make_move("e5").unwrap();
    game = game.make_move("d4").unwrap();
    game = game.make_move("d5").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. e4 e5 2. d4 d5 *", result);
}

#[test]
fn partially_complete_second_turn() {
    let mut game = Game::new();

    game = game.make_move("e4").unwrap();
    game = game.make_move("e5").unwrap();
    game = game.make_move("d4").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. e4 e5 2. d4 *", result);
}

#[test]
fn knight_move() {
    let mut game = Game::new();

    game = game.make_move("Nc3").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Nc3 *", result);
}

#[test]
fn king_move() {
    let mut game = Game::from_fen("8/8/8/4K3/8/8/8/k7 w KQkq - 0 1");

    game = game.make_move("Kd4").unwrap();
    game = game.make_move("Ka2").unwrap();
    game = game.make_move("Kd5").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Kd4 Ka2 2. Kd5 *", result);
}

#[test]
fn rook_move() {
    let mut game = Game::from_fen("8/8/8/4R3/8/8/8/r7 w KQkq - 0 1");

    game = game.make_move("Re1").unwrap();
    game = game.make_move("Ra8").unwrap();
    game = game.make_move("Ra1").unwrap();
    game = game.make_move("Rh8").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Re1 Ra8 2. Ra1 Rh8 *", result);
}

#[test]
fn bishop_move() {
    let mut game = Game::from_fen("8/8/8/4B3/8/8/8/b7 w KQkq - 0 1");

    game = game.make_move("Bh2").unwrap();
    game = game.make_move("Bh8").unwrap();
    game = game.make_move("Bb8").unwrap();
    game = game.make_move("Bb2").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Bh2 Bh8 2. Bb8 Bb2 *", result);
}

#[test]
fn queen_move() {
    let mut game = Game::from_fen("8/8/8/4Q3/8/8/8/q7 w KQkq - 0 1");

    game = game.make_move("Qh2").unwrap();
    game = game.make_move("Qh8").unwrap();
    game = game.make_move("Qb8").unwrap();
    game = game.make_move("Qb2").unwrap();
    game = game.make_move("Qb5").unwrap();
    game = game.make_move("Qh2").unwrap();
    game = game.make_move("Qb8").unwrap();
    game = game.make_move("Qa2").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Qh2 Qh8 2. Qb8 Qb2 3. Qb5 Qh2 4. Qb8 Qa2 *", result);
}

#[test]
fn attack_move_from_dynamic_movement_piece() {
    let mut game = Game::from_fen("8/8/8/3Rr3/3Rr3/8/8/8 w KQkq - 0 1");

    game = game.make_move("Rxe5").unwrap();
    game = game.make_move("Rxd4").unwrap();
    game = game.make_move("Rd5").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Rxe5 Rxd4 2. Rd5 *", result);
    assert_that!(&game.get_captured_pieces().second_player, contains_in_any_order(vec![
        PieceType::Rook
    ]));
    assert_that!(&game.get_captured_pieces().first_player, contains_in_any_order(vec![
        PieceType::Rook
    ]));
}

#[test]
fn attack_move_from_static_movement_piece() {
    let mut game = Game::from_fen("8/1n4N1/8/2N2n2/8/8/8/8 w - - 0 1");

    game = game.make_move("Nxb7").unwrap();
    game = game.make_move("Nxg7").unwrap();
    game = game.make_move("Nc5").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. Nxb7 Nxg7 2. Nc5 *", result);
    assert_that!(&game.get_captured_pieces().second_player, contains_in_any_order(vec![
        PieceType::Knight
    ]));
    assert_that!(&game.get_captured_pieces().first_player, contains_in_any_order(vec![
        PieceType::Knight
    ]));
}

#[test]
fn attack_move_from_pawn() {
    let mut game = Game::from_fen("8/8/3p2p1/2P2P2/8/8/8/8 w - - 0 1");

    game = game.make_move("cxd6").unwrap();
    game = game.make_move("gxf5").unwrap();
    game = game.make_move("d7").unwrap();
    game = game.make_move("f4").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. cxd6 gxf5 2. d7 f4 *", result);
    assert_that!(&game.get_captured_pieces().second_player, contains_in_any_order(vec![
        PieceType::Pawn
    ]));
    assert_that!(&game.get_captured_pieces().first_player, contains_in_any_order(vec![
        PieceType::Pawn
    ]));
}