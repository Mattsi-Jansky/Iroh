use chess::*;

#[test]
fn generate_e4_pawn_moves() {
    let game = ChessGame::new();

    let available_moves = game.get_available_moves();

    assert!(available_moves.contains(&Move::RegularMove { 0: (3,2), 1: (3,3) }));
    assert!(available_moves.contains(&Move::RegularMove { 0: (3,2), 1: (3,4) }));
}
