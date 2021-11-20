use test_case::test_case;

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
