use iroh::game::Game;

#[test]
fn given_first_player_should_castle_kingside() {
    let mut game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move("O-O").unwrap();
    assert_eq!("1. O-O *", game.get_pgn());
    assert_eq!("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 0 1", game.generate_fen());
}

#[test]
fn given_first_player_without_clear_path_cannot_castle_kingside() {
    let game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/2NB4/PPPP1PPP/R1BQK1NR w KQkq - 0 1");

    let result = game.make_move("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_not_in_place_cannot_castle_kingside() {
    let mut game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move("Rg1").unwrap();
    game = game.make_move("h6").unwrap();
    let result = game.make_move("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_has_moved_cannot_castle_kingside() {
    let mut game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move("Rg1").unwrap();
    game = game.make_move("a6").unwrap();
    game = game.make_move("Rh1").unwrap();
    game = game.make_move("b6").unwrap();
    let result = game.make_move("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_king_has_moved_cannot_castle_kingside() {
    let mut game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move("Kf1").unwrap();
    game = game.make_move("a6").unwrap();
    game = game.make_move("Ke1").unwrap();
    game = game.make_move("b6").unwrap();
    let result = game.make_move("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_fen_says_they_cant_castle_kingside_cannot_castle_kingside() {
    let mut game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w Qkq - 0 1");

    let result = game.make_move("O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_should_castle_queenside() {
    let mut game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w KQkq - 0 1");

    game = game.make_move("O-O-O").unwrap();
    assert_eq!("1. O-O-O *", game.get_pgn());
    assert_eq!("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/2KR1BNR b kq - 0 1", game.generate_fen());
}

#[test]
fn given_first_player_without_clear_path_cannot_castle_queenside() {
    let game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/P1PP1PPP/R1B1KBNR b KQkq - 0 1");

    let result = game.make_move("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_not_in_place_cannot_castle_queenside() {
    let mut game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w KQkq - 0 1");

    game = game.make_move("Rb1").unwrap();
    game = game.make_move("h6").unwrap();
    let result = game.make_move("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_rook_has_moved_cannot_castle_queenside() {
    let mut game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w KQkq - 0 1");

    game = game.make_move("Rb1").unwrap();
    game = game.make_move("a6").unwrap();
    game = game.make_move("Ra1").unwrap();
    game = game.make_move("b6").unwrap();
    let result = game.make_move("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_first_player_and_fen_says_they_cant_castle_queenside_cannot_castle_queenside() {
    let mut game = Game::from_fen("r1bqk2r/ppp2ppp/2nb1n2/3pp3/8/1PN1PQ2/PBPP1PPP/R3KBNR w Kkq - 0 1");

    let result = game.make_move("O-O-O");

    assert!(result.is_err());
}

#[test]
fn given_second_player_should_castle_kingside() {
    let mut game = Game::from_fen("rnbqk2r/ppppbppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 1");

    game = game.make_move("e4").unwrap();
    game = game.make_move("O-O").unwrap();
    assert_eq!("1. e4 O-O *", game.get_pgn());
    assert_eq!("rnbq1rk1/ppppbppp/4pn2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R w KQ - 0 1", game.generate_fen());
}