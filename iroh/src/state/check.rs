use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn is_check(is_first_player: bool, game_state: &GameState) -> bool{
    let mut result = false;

    let king = game_state.board.get_all_pieces_belonging_to_player(is_first_player)
        .into_iter()
        .skip_while(|piece| piece.0 != PieceType::King)
        .next();

    if let Some(king) = king {
        [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)].map(|transformation| {
            let target_file = king.1.transform(transformation.0);
            let target_rank = king.2.transform(transformation.1);

            if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
                if let Some(piece) = game_state.board[(target_file, target_rank)] {
                    if piece.is_owned_by_first_player != is_first_player
                        && piece.piece_type == PieceType::King { result = true; }
                }
            }
        });

        [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)].map(|transformation| {
            let target_file = king.1.transform(transformation.0);
            let target_rank = king.2.transform(transformation.1);

            if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
                if let Some(piece) = game_state.board[(target_file, target_rank)] {
                    if piece.is_owned_by_first_player != is_first_player
                        && piece.piece_type == PieceType::Knight { result = true; }
                }
            }
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::GameState;

    #[test]
    fn no_check() {
        let game_state = GameState::from_fen("8/8/8/8/8/8/4K3/8 w - - 0 1");

        let result = is_check(true, &game_state);

        assert_eq!(false, result);
    }

    #[test]
    fn king_check() {
        let game_state = GameState::from_fen("8/8/8/8/8/3k4/4K3/8 w - - 0 1");

        let result = is_check(true, &game_state);

        assert_eq!(true, result);
    }

    #[test]
    fn knight_check() {
        let game_state = GameState::from_fen("8/8/8/3n4/8/4K3/8/8 w - - 0 1");

        let result = is_check(true, &game_state);

        assert_eq!(true, result);
    }
}
