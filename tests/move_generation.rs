use test_case::test_case;

use chess::*;

#[test_case("a2", 0)]
#[test_case("e2", 3)]
fn generate_first_turn_pawn_moves(tile: &str, rank: u8) {
    let game = ChessGame::new();

    let available_moves = game.get_available_moves(tile);

    assert!(available_moves.contains(&Move::RegularMove { 0: (rank,2), 1: (rank,3) }));
    assert!(available_moves.contains(&Move::RegularMove { 0: (rank,2), 1: (rank,4) }));
}