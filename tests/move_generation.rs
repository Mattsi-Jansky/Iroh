#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::collection::*;
use test_case::test_case;

use chess::moves::Move;
use chess::game::Game;

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(6)]
#[test_case(7)]
fn generate_first_turn_pawn_moves(rank: usize) {
    let game = Game::new();

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_subset(vec![
        Move::PawnMove { 0: rank, 1: (rank,2) },
        Move::PawnMove { 0: rank, 1: (rank,3) },
    ]));
}

#[test]
fn generate_moves_for_knight() {
    let game = Game::from_fen("8/8/8/4N3/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_subset(vec![
        Move::RegularMove { 0: (4,4), 1: (2,3) },
        Move::RegularMove { 0: (4,4), 1: (2,5) },

        Move::RegularMove { 0: (4,4), 1: (3,2) },
        Move::RegularMove { 0: (4,4), 1: (3,6) },

        Move::RegularMove { 0: (4,4), 1: (6,3) },
        Move::RegularMove { 0: (4,4), 1: (6,5) },

        Move::RegularMove { 0: (4,4), 1: (5,2) },
        Move::RegularMove { 0: (4,4), 1: (5,6) },
    ]));
}
