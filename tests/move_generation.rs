#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::collection::*;
use test_case::test_case;

use chess::piece::PieceType;
use chess::moves::Move;
use chess::game::Game;
use chess::coordinates::{Rank,File};

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(6)]
#[test_case(7)]
fn generate_first_turn_pawn_moves(file: usize) {
    let rank = File::new(file);
    let game = Game::new();

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_subset(vec![
        Move::PawnMove { 0: rank, 1: (rank,Rank::new(2)) },
        Move::PawnMove { 0: rank, 1: (rank,Rank::new(3)) },
    ]));
}

#[test]
fn generate_moves_for_knight() {
    let game = Game::from_fen("8/8/8/4N3/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(3)), 2: PieceType::Knight },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(5)), 2: PieceType::Knight },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(2)), 2: PieceType::Knight },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(6)), 2: PieceType::Knight },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(3)), 2: PieceType::Knight },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(5)), 2: PieceType::Knight },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(2)), 2: PieceType::Knight },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(6)), 2: PieceType::Knight },
    ]));
}

#[test]
fn knight_moves_do_not_go_off_edge_of_board() {
    let game = Game::from_fen("8/8/8/8/8/8/8/N7 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::RegularMove { 0: (File::new(0),Rank::new(0)), 1: (File::new(1),Rank::new(2)), 2: PieceType::Knight },
        Move::RegularMove { 0: (File::new(0),Rank::new(0)), 1: (File::new(2),Rank::new(1)), 2: PieceType::Knight },
    ]));
}
