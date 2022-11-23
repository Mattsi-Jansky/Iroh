#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::collection::*;
use iroh::game::Game;
use test_case::test_case;

use iroh::moves::Move;
use iroh::state::coordinates::Coordinate;
use iroh::state::tile::Tile;

#[test_case(Coordinate::A2)]
#[test_case(Coordinate::B2)]
#[test_case(Coordinate::C2)]
#[test_case(Coordinate::D2)]
#[test_case(Coordinate::E2)]
#[test_case(Coordinate::F2)]
#[test_case(Coordinate::G2)]
#[test_case(Coordinate::H2)]
fn generate_first_turn_pawn_moves(from: Coordinate) {
    let game = Game::new().unwrap_if_ongoing();
    let one_ahead = from.north().unwrap();
    let two_ahead = one_ahead.north().unwrap();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_subset(vec![
            Move::PawnMove {
                0: from,
                1: one_ahead
            },
            Move::PawnMove {
                0: from,
                1: two_ahead
            },
        ])
    );
}

#[test]
fn generate_moves_for_knight() {
    let game = Game::from_fen("8/8/8/4N3/8/8/8/8 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C4,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C6,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D3,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D7,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G4,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G6,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F3,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F7,
                2: Tile::FIRST_KNIGHT
            },
        ])
    );
}

#[test]
fn knight_moves_do_not_go_off_edge_of_board() {
    let game = Game::from_fen("8/8/8/8/8/8/8/N7 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::RegularMove {
                0: Coordinate::A1,
                1: Coordinate::B3,
                2: Tile::FIRST_KNIGHT
            },
            Move::RegularMove {
                0: Coordinate::A1,
                1: Coordinate::C2,
                2: Tile::FIRST_KNIGHT
            },
        ])
    );
}

#[test]
fn generate_moves_for_king() {
    let game = Game::from_fen("8/8/8/4K3/8/8/8/8 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D4,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D5,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D6,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E4,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E6,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F4,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F5,
                2: Tile::FIRST_KING
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F6,
                2: Tile::FIRST_KING
            },
        ])
    );
}

#[test]
fn generate_moves_for_rook() {
    let game = Game::from_fen("8/8/8/4R3/8/8/8/8 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E1,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E2,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E3,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E4,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E6,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E7,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E8,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::A5,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::B5,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C5,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D5,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F5,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G5,
                2: Tile::FIRST_ROOK
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::H5,
                2: Tile::FIRST_ROOK
            },
        ])
    );
}

#[test]
fn generate_moves_for_bishop() {
    let game = Game::from_fen("8/8/8/4B3/8/8/8/8 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::A1,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::B2,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C3,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D4,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F6,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G7,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::H8,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::H2,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G3,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F4,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D6,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C7,
                2: Tile::FIRST_BISHOP
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::B8,
                2: Tile::FIRST_BISHOP
            },
        ])
    );
}

#[test]
fn generate_moves_for_queen() {
    let game = Game::from_fen("8/8/8/4Q3/8/8/8/8 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::A1,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::B2,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C3,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D4,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F6,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G7,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::H8,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::H2,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G3,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F4,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D6,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::B8,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C7,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E1,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E2,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E3,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E4,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E6,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E7,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::E8,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::A5,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::B5,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::C5,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::D5,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::F5,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::G5,
                2: Tile::FIRST_QUEEN
            },
            Move::RegularMove {
                0: Coordinate::E5,
                1: Coordinate::H5,
                2: Tile::FIRST_QUEEN
            },
        ])
    );
}

#[test]
fn pawn_attack_moves() {
    let game = Game::from_fen("8/8/2p1p3/3P4/8/8/8/8 w KQkq - 0 1").unwrap_if_ongoing();

    let available_moves = game.get_available_moves();

    assert_that!(
        &available_moves,
        contains_in_any_order(vec![
            Move::PawnAttackMove {
                0: Coordinate::D5,
                1: Coordinate::C6
            },
            Move::PawnMove {
                0: Coordinate::D5,
                1: Coordinate::D6
            },
            Move::PawnAttackMove {
                0: Coordinate::D5,
                1: Coordinate::E6
            },
        ])
    );
}
