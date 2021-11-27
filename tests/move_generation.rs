#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::collection::*;
use test_case::test_case;

use chess::state::piece::PieceType;
use chess::moves::Move;
use chess::game::Game;
use chess::state::coordinates::{Rank, File};

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(6)]
#[test_case(7)]
fn generate_first_turn_pawn_moves(file: usize) {
    let file = File::new(file);
    let game = Game::new();

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_subset(vec![
        Move::PawnMove { 0: file, 1: Rank::new(2) },
        Move::PawnMove { 0: file, 1: Rank::new(3) },
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

#[test]
fn generate_moves_for_king() {
    let game = Game::from_fen("8/8/8/4K3/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(3)), 2: PieceType::King },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(4)), 2: PieceType::King },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(5)), 2: PieceType::King },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(3)), 2: PieceType::King },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(5)), 2: PieceType::King },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(3)), 2: PieceType::King },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(4)), 2: PieceType::King },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(5)), 2: PieceType::King },
    ]));
}

#[test]
fn generate_moves_for_rook() {
    let game = Game::from_fen("8/8/8/4R3/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(0)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(1)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(2)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(3)), 2: PieceType::Rook },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(5)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(6)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(7)), 2: PieceType::Rook },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(0),Rank::new(4)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(1),Rank::new(4)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(4)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(4)), 2: PieceType::Rook },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(4)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(4)), 2: PieceType::Rook },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(7),Rank::new(4)), 2: PieceType::Rook },
    ]));
}

#[test]
fn generate_moves_for_bishop() {
    let game = Game::from_fen("8/8/8/4B3/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(0),Rank::new(0)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(1),Rank::new(1)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(2)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(3)), 2: PieceType::Bishop },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(5)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(6)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(7),Rank::new(7)), 2: PieceType::Bishop },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(7),Rank::new(1)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(2)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(3)), 2: PieceType::Bishop },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(5)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(6)), 2: PieceType::Bishop },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(1),Rank::new(7)), 2: PieceType::Bishop },
    ]));
}

#[test]
fn generate_moves_for_queen() {
    let game = Game::from_fen("8/8/8/4Q3/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(0),Rank::new(0)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(1),Rank::new(1)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(2)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(3)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(5)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(6)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(7),Rank::new(7)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(7),Rank::new(1)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(2)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(3)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(5)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(6)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(1),Rank::new(7)), 2: PieceType::Queen },

        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(0)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(1)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(2)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(3)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(5)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(6)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(4),Rank::new(7)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(0),Rank::new(4)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(1),Rank::new(4)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(2),Rank::new(4)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(3),Rank::new(4)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(5),Rank::new(4)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(6),Rank::new(4)), 2: PieceType::Queen },
        Move::RegularMove { 0: (File::new(4),Rank::new(4)), 1: (File::new(7),Rank::new(4)), 2: PieceType::Queen },
    ]));
}

#[test]
fn pawn_attack_moves() {
    let game = Game::from_fen("8/8/2p1p3/3P4/8/8/8/8 w KQkq - 0 1");

    let available_moves = game.get_available_moves();

    assert_that!(&available_moves, contains_in_any_order(vec![
        Move::PawnAttackMove { 0: File::new(3), 1: (File::new(2),Rank::new(5)) },
        Move::PawnMove { 0: File::new(3), 1: Rank::new(5) },
        Move::PawnMove { 0: File::new(3), 1: Rank::new(6) },
        Move::PawnAttackMove { 0: File::new(3), 1: (File::new(4),Rank::new(5)) },
    ]));
}