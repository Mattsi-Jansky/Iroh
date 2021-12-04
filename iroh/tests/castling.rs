use iroh::game::Game;

#[test]
fn given_first_player_should_castle_kingside() {
    let mut game = Game::from_fen("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");

    game = game.make_move("O-O").unwrap();
    assert_eq!("1. O-O *", game.get_pgn());
    assert_eq!("rnbqk1nr/ppp1bppp/3p4/4p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 0 1", game.generate_fen());
}
