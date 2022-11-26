use iroh::game::Game;
use test_case::test_case;

#[test]
fn given_first_player_should_castle_kingside() {
    let game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    let game = game.make_move_san("O-O");
    assert_eq!("1. O-O *", game.generate_pgn().unwrap());
    assert_eq!(
        "rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 0 1",
        game.generate_fen().unwrap()
    );
}

#[test]
fn given_first_player_without_clear_path_cannot_castle_kingside() {
    let game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/2NB4/PPPP1PPP/R1BQK1NR w KQkq - 0 1");

    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_not_in_place_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move_san("Rg1");
    game = game.make_move_san("h6");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_has_moved_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move_san("Rg1");
    game = game.make_move_san("a6");
    game = game.make_move_san("Rh1");
    game = game.make_move_san("b6");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_king_has_moved_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move_san("Kf1");
    game = game.make_move_san("a6");
    game = game.make_move_san("Ke1");
    game = game.make_move_san("b6");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_fen_says_they_cant_castle_kingside_cannot_castle_kingside() {
    let game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w Qkq - 0 1");

    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_should_castle_queenside() {
    let game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w KQkq - 0 1");

    let game = game.make_move_san("O-O-O");
    assert_eq!("1. O-O-O *", game.generate_pgn().unwrap());
    assert_eq!(
        "r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/2KR1BNR b kq - 0 1",
        game.generate_fen().unwrap()
    );
}

#[test]
fn given_first_player_without_clear_path_cannot_castle_queenside() {
    let game =
        Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/P1PP1PPP/R1B1KBNR b KQkq - 0 1");

    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_not_in_place_cannot_castle_queenside() {
    let mut game =
        Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w KQkq - 0 1");

    game = game.make_move_san("Rb1");
    game = game.make_move_san("h6");
    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_has_moved_cannot_castle_queenside() {
    let mut game =
        Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w KQkq - 0 1");

    game = game.make_move_san("Rb1");
    game = game.make_move_san("a6");
    game = game.make_move_san("Ra1");
    game = game.make_move_san("b6");
    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_fen_says_they_cant_castle_queenside_cannot_castle_queenside() {
    let game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w Kkq - 0 1");

    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_should_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk2r/ppppbppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 1");

    game = game.make_move_san("e4");
    let game = game.make_move_san("O-O");

    assert_eq!("1. e4 O-O *", game.generate_pgn().unwrap());
    assert_eq!(
        "rnbq1rk1/ppppbppp/4pn2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R w KQ - 0 1",
        game.generate_fen().unwrap()
    );
}

#[test]
fn given_second_player_without_clear_path_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqkb1r/pppp1ppp/4pn2/8/3P4/2N1PN2/PPP2PPP/R1BQKB1R w KQkq - 1 2");

    game = game.make_move_san("e4");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_rook_not_in_place_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk2r/ppppbppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 1");

    game = game.make_move_san("e4");
    game = game.make_move_san("Rg8");
    game = game.make_move_san("e5");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_rook_has_moved_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk2r/ppppbppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 1");

    game = game.make_move_san("e4");
    game = game.make_move_san("Rg8");
    game = game.make_move_san("e5");
    game = game.make_move_san("Rh8");
    game = game.make_move_san("a3");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_king_has_moved_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk2r/ppppbppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 1");

    game = game.make_move_san("e4");
    game = game.make_move_san("Kf8");
    game = game.make_move_san("e5");
    game = game.make_move_san("Ke8");
    game = game.make_move_san("a3");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_fen_says_they_cant_castle_kingside_cannot_castle_kingside() {
    let mut game =
        Game::from_fen("rnbqk2r/ppppbppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQq - 0 1");

    game = game.make_move_san("e4");
    let result = game.make_move_san("O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_should_castle_queenside() {
    let mut game =
        Game::from_fen("r3kbnr/pbpp1ppp/1pn1pq2/8/3PP3/2NB1N2/PPP2PPP/R1BQK2R w KQkq - 0 1");

    game = game.make_move_san("a3");
    let game = game.make_move_san("O-O-O");

    assert_eq!("1. a3 O-O-O *", game.generate_pgn().unwrap());
    assert_eq!(
        "2kr1bnr/pbpp1ppp/1pn1pq2/8/3PP3/P1NB1N2/1PP2PPP/R1BQK2R w KQ - 0 1",
        game.generate_fen().unwrap()
    );
}

#[test]
fn given_second_player_without_clear_path_cannot_castle_queenside() {
    let mut game =
        Game::from_fen("r1b1kbnr/p1pp1ppp/1pn1pq2/8/3PP3/P1NB1N2/1PP2PPP/R1BQK2R w KQkq - 1 2");

    game = game.make_move_san("a4");
    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_rook_not_in_place_cannot_castle_queenside() {
    let mut game =
        Game::from_fen("r3kbnr/pbpp1ppp/1pn1pq2/8/3PP3/2NB1N2/PPP2PPP/R1BQK2R w KQkq - 0 1");

    game = game.make_move_san("a3");
    game = game.make_move_san("Rb8");
    game = game.make_move_san("a4");
    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_rook_has_moved_cannot_castle_queenside() {
    let mut game =
        Game::from_fen("r3kbnr/pbpp1ppp/1pn1pq2/8/3PP3/2NB1N2/PPP2PPP/R1BQK2R w KQkq - 0 1");

    game = game.make_move_san("a3");
    game = game.make_move_san("Rb8");
    game = game.make_move_san("a4");
    game = game.make_move_san("Ra8");
    game = game.make_move_san("b3");
    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_and_fen_says_they_cant_castle_queenside_cannot_castle_queenside() {
    let mut game =
        Game::from_fen("r3kbnr/pbpp1ppp/1pn1pq2/8/3PP3/2NB1N2/PPP2PPP/R1BQK2R w KQk - 0 1");

    game = game.make_move_san("a3");
    let result = game.make_move_san("O-O-O");

    assert!(result.is_err());
}

#[test_case("rnb1kbnr/pppppppp/5q2/8/8/8/PPPPP3/RNBQK2R w KQkq - 0 1", "O-O")]
#[test_case("rnb1kbnr/pppppppp/6q1/8/8/8/PPPPP3/RNBQK2R w KQkq - 0 1", "O-O")]
#[test_case("rnb1kbnr/pppppppp/3q4/8/8/8/P3PPPP/R3KBNR w KQkq - 0 1", "O-O-O")]
#[test_case("rnb1kbnr/pppppppp/2q5/8/8/8/P3PPPP/R3KBNR w KQkq - 0 1", "O-O-O")]
#[test_case("rnbqk2r/ppppp2p/8/8/8/5Q2/PPPPPPPP/RNB1KBNR b KQkq - 0 1", "O-O")]
#[test_case("rnbqk2r/ppppp2p/8/8/8/6Q1/PPPPPPPP/RNB1KBNR b KQkq - 0 1", "O-O")]
#[test_case("r3kbnr/p3pppp/8/8/8/3Q4/PPPPPPPP/RNB1KBNR b KQkq - 0 1", "O-O-O")]
#[test_case("r3kbnr/p3pppp/8/8/8/2Q5/PPPPPPPP/RNB1KBNR b KQkq - 0 1", "O-O-O")]
fn given_king_would_go_through_attacked_tile_cannot_castle(fen: &str, sen: &str) {
    let game = Game::from_fen(fen);

    let result = game.make_move_san(sen);

    assert!(result.is_err());
}
