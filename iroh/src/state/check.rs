use crate::moves::{KING_STATIC_TRANSFORMS, KNIGHT_STATIC_TRANSFORMS};
use crate::state::board::Board;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub fn is_check(is_first_player: bool, game_state: &GameState) -> bool{
    let mut result = false;

    let king = game_state.board.get_all_pieces_belonging_to_player(is_first_player)
        .into_iter()
        .find(|piece| piece.0 == PieceType::King);

    if let Some(king) = king {
        let mut static_check = StaticCheckTester::new(&mut result, is_first_player, &game_state.board, &king);
        static_check.test(PieceType::King, &KING_STATIC_TRANSFORMS);
        static_check.test(PieceType::Knight, &KNIGHT_STATIC_TRANSFORMS);
        static_check.test(PieceType::Pawn, &[(-1, -1), (1, -1)]);
    }

    result
}

struct StaticCheckTester<'a> {
    result: &'a mut bool,
    is_first_player: bool,
    board: &'a Board,
    king: &'a (PieceType, File, Rank)
}

impl<'a> StaticCheckTester<'a> {
    fn new(result: &'a mut bool,
           is_first_player: bool,
           board: &'a Board,
           king: &'a (PieceType, File, Rank)) -> StaticCheckTester<'a> {
        StaticCheckTester {result, is_first_player, board, king}
    }

    fn test(&mut self, attacking_piece_type: PieceType, transformations: &[(isize, isize)]) {
        for transform in transformations {
            let target_file = self.king.1.transform(transform.0);
            let target_rank = self.king.2.transform(transform.1);

            if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
                if let Some(piece) = self.board[(target_file, target_rank)] {
                    if piece.is_owned_by_first_player != self.is_first_player
                        && piece.piece_type == attacking_piece_type { *self.result = true; }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::GameState;

    macro_rules! check_tests {
        ($($name:ident {$fen:expr;$expected:expr}),+) => {
            $(#[test]
            fn $name() {
                let game_state = GameState::from_fen($fen);

                let result = is_check(true, &game_state);

                assert_eq!($expected, result);
            })+
        }
    }

    check_tests! {
        no_check {"8/8/8/8/8/8/4K3/4p3 w - - 0 1";false},
        king_check {"8/8/8/8/8/3k4/4K3/8 w - - 0 1";true},
        knight_check {"8/8/8/3n4/8/4K3/8/8 w - - 0 1";true},
        pawn_check {"8/8/8/8/8/4K3/3p4/8 w - - 0 1";true}
    }
}

