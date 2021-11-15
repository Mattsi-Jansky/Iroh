#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::*;
use galvanic_assert::matchers::collection::*;
use test_case::test_case;

use chess::*;

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(6)]
#[test_case(7)]
fn generate_first_turn_pawn_moves(rank: u8) {
    let game = ChessGame::new();

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_subset(vec![
        Move::PawnMove { 0: rank, 1: (rank,3) },
        Move::PawnMove { 0: rank, 1: (rank,4) },
    ]));
}
