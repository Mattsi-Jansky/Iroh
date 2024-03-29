#[macro_use]
extern crate galvanic_assert;
extern crate core;

use galvanic_assert::matchers::collection::*;
use iroh::game::Game;
use iroh::state::tile::Tile;
use test_case::test_case;

mod generators;

#[test]
fn new_game_pgn_has_asterisk_only() {
    let game = Game::new();

    let result = game.generate_pgn().unwrap();

    assert_eq!(result, "*");
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
    let game = Game::new();

    let game = game.make_move_san(san);
    let result = game.generate_pgn().unwrap();

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

    game = game.make_move_san("e4");
    let game = game.make_move_san(san);
    let result = game.generate_pgn().unwrap();

    assert_eq!(format!("1. e4 {} *", san), result);
}

chess_test! {
    {second_turn,["e4","e5","d4","d5"],"1. e4 e5 2. d4 d5 *"}
    {partially_complete_second_turn,["e4","e5","d4"],"1. e4 e5 2. d4 *"}
    {knight_move,["Nc3"],"1. Nc3 *"}
    {king_move @ "8/8/8/4K3/8/8/8/k7 w KQkq - 0 1",["Kd4", "Ka2", "Kd5"],
        "1. Kd4 Ka2 2. Kd5 *"}
    {rook_move @ "8/8/8/4R3/8/8/8/r7 w KQkq - 0 1",["Re1","Ra8","Ra1","Rh8"],
        "1. Re1 Ra8 2. Ra1 Rh8 *"}
    {bishop_move @ "8/8/8/4B3/8/8/8/b7 w KQkq - 0 1",["Bh2","Bh8","Bb8","Bb2"],
        "1. Bh2 Bh8 2. Bb8 Bb2 *"}
    {queen_move @ "8/8/8/4Q3/8/8/8/q7 w KQkq - 0 1",
        ["Qh2","Qh8","Qb8","Qb2","Qb5","Qh2","Qb8","Qa2"],
        "1. Qh2 Qh8 2. Qb8 Qb2 3. Qb5 Qh2 4. Qb8 Qa2 *"}
    {promote_to_queen @ "8/3P4/8/8/8/8/8/8 w - - 0 1",["d8=Q"],
        "1. d8=Q 1/2-1/2","3Q4/8/8/8/8/8/8/8 b - - 0 1"}
    {promote_to_knight @ "8/3P4/8/8/8/8/8/8 w - - 0 1", ["d8=N"],"1. d8=N 1/2-1/2",
        "3N4/8/8/8/8/8/8/8 b - - 0 1"}
    {promote_to_bishop @ "8/3P4/8/8/8/8/8/8 w - - 0 1",["d8=B"],"1. d8=B 1/2-1/2",
        "3B4/8/8/8/8/8/8/8 b - - 0 1"}
    {promote_to_rook @ "8/3P4/8/8/8/8/8/8 w - - 0 1",["d8=R"],
        "1. d8=R 1/2-1/2","3R4/8/8/8/8/8/8/8 b - - 0 1"}
    {promote_second_player_pawn_to_queen @ "8/8/3P4/8/8/8/3p4/8 w - - 0 1",["d7","d1=Q"],
        "1. d7 d1=Q *","8/3P4/8/8/8/8/8/3q4 w - - 0 1"}
}

#[test]
fn attack_move_from_dynamic_movement_piece() {
    let mut game = Game::from_fen("8/8/8/3Rr3/3Rr3/8/8/8 w KQkq - 0 1");

    game = game.make_move_san("Rxe5");
    game = game.make_move_san("Rxd4");
    let game = game.make_move_san("Rd5");
    let result_pgn = game.generate_pgn().unwrap();
    let result_game = game;

    assert_eq!("1. Rxe5 Rxd4 2. Rd5 *", result_pgn);
    assert_that!(
        &result_game.captured_pieces().unwrap().second_player,
        contains_in_any_order(vec![Tile::SECOND_ROOK])
    );
    assert_that!(
        &result_game.captured_pieces().unwrap().first_player,
        contains_in_any_order(vec![Tile::FIRST_ROOK])
    );
}

#[test]
fn attack_move_from_static_movement_piece() {
    let mut game = Game::from_fen("8/1n4N1/8/2N2n2/8/8/8/8 w - - 0 1");

    game = game.make_move_san("Nxb7");
    game = game.make_move_san("Nxg7");
    let game = game.make_move_san("Nc5");
    let result = game.generate_pgn().unwrap();
    let result_game = game;

    assert_eq!("1. Nxb7 Nxg7 2. Nc5 *", result);
    assert_that!(
        &result_game.captured_pieces().unwrap().second_player,
        contains_in_any_order(vec![Tile::SECOND_KNIGHT])
    );
    assert_that!(
        &result_game.captured_pieces().unwrap().first_player,
        contains_in_any_order(vec![Tile::FIRST_KNIGHT])
    );
}

#[test]
fn attack_move_from_pawn() {
    let mut game = Game::from_fen("8/8/3p2p1/2P2P2/8/8/8/8 w - - 0 1");

    game = game.make_move_san("cxd6");
    game = game.make_move_san("gxf5");
    game = game.make_move_san("d7");
    let game = game.make_move_san("f4");
    let result = game.generate_pgn().unwrap();
    let result_game = game;

    assert_eq!("1. cxd6 gxf5 2. d7 f4 *", result);
    assert_that!(
        &result_game.captured_pieces().unwrap().second_player,
        contains_in_any_order(vec![Tile::SECOND_PAWN])
    );
    assert_that!(
        &result_game.captured_pieces().unwrap().first_player,
        contains_in_any_order(vec![Tile::FIRST_PAWN])
    );
}

#[test]
fn generate_fen_from_game() {
    let game = Game::new();

    let result = game.generate_fen().unwrap();

    assert_eq!(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        result
    );
}

#[test]
fn given_current_player_can_move_game_is_ongoing() {
    let game = Game::from_fen("5k2/R7/8/8/8/8/8/1R2K3 w - - 0 1");

    assert!(matches!(game, Game::Ongoing { .. }));
}

#[test]
fn checkmate_second_player() {
    let game = Game::from_fen("5k2/R7/8/8/8/8/8/1R2K3 w - - 0 1");
    assert!(matches!(game, Game::Ongoing { .. }));

    let result = game.make_move_san("Rb8");

    assert_eq!("1. Rb8 1-0", result.generate_pgn().unwrap());
    assert!(matches!(
        result,
        Game::Win {
            is_first_player_win: true,
            ..
        }
    ));
}

#[test]
fn checkmate_first_player() {
    let game = Game::from_fen("1r3k2/8/8/8/8/8/r7/4K3 b - - 0 1");
    assert!(matches!(game, Game::Ongoing { .. }));

    let game = game.make_move_san("Rb1");

    assert_eq!("1. Rb1 0-1", game.generate_pgn().unwrap());
    assert!(matches!(
        game,
        Game::Win {
            is_first_player_win: false,
            ..
        }
    ));
}

#[test]
fn given_stalemate_should_automatically_draw() {
    let game = Game::from_fen("1N6/8/2R5/3k4/4R3/8/5N2/3K4 b - - 0 1");

    assert!(matches!(game, Game::Draw { .. }));
    assert_eq!("1/2-1/2", game.generate_pgn().unwrap());
}

#[test]
fn given_five_fold_repetition_from_second_player_should_automatically_draw() {
    //Example from Kasparov versus Deep Blue, 1997
    let mut game = Game::from_fen("6Q1/pp6/8/8/1kp2N2/1n2R1P1/3r4/1K6 b - - 21 12");

    game = game.make_move_san("Rd1");
    game = game.make_move_san("Kb2");
    game = game.make_move_san("Rd2");
    game = game.make_move_san("Kb1");
    game = game.make_move_san("Rd1");
    game = game.make_move_san("Kc2");
    game = game.make_move_san("Rd2");
    game = game.make_move_san("Kb1");
    game = game.make_move_san("Rd1");

    assert!(matches!(game, Game::Draw { .. }));
    assert_eq!(
        "1. Rd1 Kb2 2. Rd2 Kb1 3. Rd1 Kc2 4. Rd2 Kb1 5. Rd1 1/2-1/2",
        game.generate_pgn().unwrap()
    );
}

#[test]
fn given_five_fold_repetition_from_first_player_should_automatically_draw() {
    let mut game = Game::from_fen("6q1/PP6/8/8/1KP2n2/1N2r1p1/3R4/1k6 w - - 21 12");

    game = game.make_move_san("Rd1");
    game = game.make_move_san("Kb2");
    game = game.make_move_san("Rd2");
    game = game.make_move_san("Kb1");
    game = game.make_move_san("Rd1");
    game = game.make_move_san("Kc2");
    game = game.make_move_san("Rd2");
    game = game.make_move_san("Kb1");
    game = game.make_move_san("Rd1");

    assert!(matches!(game, Game::Draw { .. }));
    assert_eq!(
        "1. Rd1 Kb2 2. Rd2 Kb1 3. Rd1 Kc2 4. Rd2 Kb1 5. Rd1 1/2-1/2",
        game.generate_pgn().unwrap()
    );
}

#[test]
fn given_seventy_five_turns_without_pawn_move_or_capture_should_automatically_draw() {
    let mut game = Game::from_fen("4k3/3rr3/8/8/8/8/3RR3/4K3 w - - 0 1");

    let moves = vec!["Rc2", "Rc7", "Rb2", "Rb7", "Ra2", "Ra7"];
    let mut index = 0;

    for _ in 0..74 {
        if index == moves.len() {
            index = 0;
        }
        game = game.make_move_san(moves[index]);
        index += 1;
    }

    assert!(matches!(game, Game::Draw { .. }));
}

#[test]
fn en_passant() {
    let mut game = Game::from_fen("3k4/2p5/8/1P6/8/8/8/3K4 b - - 0 1");

    let game = game.make_move_san("c5");
    let game = game.make_move_san("bxc6");

    assert_eq!(game.generate_fen().unwrap(), "3k4/8/2P5/8/8/8/8/3K4 b - - 0 1");
    assert_eq!(game.generate_pgn().unwrap(), "1. c5 bxc6 *");
}
