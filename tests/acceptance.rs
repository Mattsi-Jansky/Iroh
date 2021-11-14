use test_case::test_case;

use chess::*;

#[test]
fn new_game_has_blank_pgn() {
    let game = ChessGame::new();

    let result = game.get_pgn();

    assert_eq!(result,"");
}

#[test_case("a4")]
#[test_case("b4")]
#[test_case("c4")]
#[test_case("d4")]
#[test_case("e4")]
#[test_case("f4")]
#[test_case("g4")]
#[test_case("h4")]
fn first_pawn_move_recorded_in_pgn(san: &str) {
    let mut game = ChessGame::new();

    game = game.make_move(san).unwrap();
    let result = game.get_pgn();

    assert_eq!(format!("1. {} *", san), result);
}
