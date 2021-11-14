#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::*;
use galvanic_assert::matchers::collection::*;
use test_case::test_case;

use chess::*;

#[test_case("a2", 0)]
#[test_case("b2", 1)]
#[test_case("c2", 2)]
#[test_case("d2", 3)]
#[test_case("e2", 4)]
#[test_case("f2", 5)]
#[test_case("g2", 6)]
#[test_case("h2", 7)]
fn generate_first_turn_pawn_moves(tile: &str, rank: u8) {
    let game = ChessGame::new();

    let available_moves = game.get_available_moves(tile);

    assert_that!(&available_moves, contains_subset(vec![
        Move::RegularMove { 0: (rank,2), 1: (rank,3) },
        Move::RegularMove { 0: (rank,2), 1: (rank,4) },
    ]));
}
