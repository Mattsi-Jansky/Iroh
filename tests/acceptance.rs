use chess::*;

#[test]
fn new_game_has_blank_pgn() {
    let game = ChessGame::new();

    let result = game.get_pgn();

    assert_eq!(result,"");
}

#[test]
fn pawn_move_recorded_in_pgn() {
    let mut game = ChessGame::new();

    game = game.make_move("d4").unwrap();
    let result = game.get_pgn();

    assert_eq!("1. d4 *", result);
}
