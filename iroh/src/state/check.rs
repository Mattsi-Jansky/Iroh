use crate::moves::{DIAGONAL_DYNAMIC_TRANSFORMS, KING_STATIC_TRANSFORMS, KNIGHT_STATIC_TRANSFORMS, STRAIGHT_DYNAMIC_TRANSFORMS};
use crate::state::board::Board;
use crate::state::coordinates::Coordinate;
use crate::state::GameState;
use crate::state::tile::{Tile};

pub fn is_check(is_first_player: bool, game_state: &GameState) -> bool{
    // let mut result = false;
    //
    // let king = game_state.board.get_all_pieces_belonging_to_player(is_first_player)
    //     .into_iter()
    //     .find(|piece| piece.0 == Tile::FIRST_KING || piece.0 == Tile::SECOND_KING);
    //
    // if let Some(king) = king {
    //     let mut static_check = StaticCheckTester::new(&mut result, is_first_player, &game_state.board, &king);
    //     static_check.test(if is_first_player { Tile::SECOND_KING } else { Tile::FIRST_KING }, &KING_STATIC_TRANSFORMS);
    //     static_check.test(if is_first_player { Tile::SECOND_KNIGHT } else { Tile::FIRST_KNIGHT }, &KNIGHT_STATIC_TRANSFORMS);
    //     static_check.test(if is_first_player { Tile::SECOND_PAWN} else { Tile::FIRST_PAWN},
    //                       if is_first_player { &[(-1, 1), (1, 1)]}
    //                       else {&[(-1, -1), (1, -1)]});
    //
    //     let mut dynamic_check = DynamicCheckTester::new(&mut result, is_first_player, &game_state.board, &king);
    //     dynamic_check.test_straight_lines();
    //     dynamic_check.test_diagonal_lines();
    // }
    //
    // result
    false
}

// struct DynamicCheckTester<'a> {
//     result: &'a mut bool,
//     board: &'a Board,
//     king: &'a (Tile, Coordinate),
//     enemy_queen: Tile,
//     enemy_rook: Tile,
//     enemy_bishop: Tile,
// }
//
// impl<'a> DynamicCheckTester<'a> {
//     fn new(result: &'a mut bool,
//            is_first_player: bool,
//            board: &'a Board,
//            king: &'a (Tile, Coordinate)) -> DynamicCheckTester<'a> {
//         DynamicCheckTester { result, board, king,
//             enemy_queen: if is_first_player { Tile::SECOND_QUEEN } else { Tile::FIRST_QUEEN },
//             enemy_rook: if is_first_player { Tile::SECOND_ROOK } else { Tile::FIRST_ROOK },
//             enemy_bishop: if is_first_player { Tile::SECOND_BISHOP } else { Tile::FIRST_BISHOP },
//         }
//     }
//
//     fn test_straight_lines(&mut self) {
//         self.test(&[self.enemy_rook, self.enemy_queen], &STRAIGHT_DYNAMIC_TRANSFORMS);
//     }
//
//     fn test_diagonal_lines(&mut self) {
//         self.test(&[self.enemy_bishop, self.enemy_queen], &DIAGONAL_DYNAMIC_TRANSFORMS);
//     }
//
//     fn test(&mut self, attacking_piece_types: &[Tile], transformations: &[(isize, isize)]) {
//         for transform in transformations {
//             let (mut file, mut rank) = (Some(self.king.1), Some(self.king.2));
//             loop {
//                 file = file.unwrap().transform(transform.0);
//                 rank = rank.unwrap().transform(transform.1);
//                 if let(Some(file), Some(rank)) = Coordinate {
//                     let tile = self.board[Coordinate];
//                     if tile.is_occupied()  {
//                         if attacking_piece_types.contains(&tile) {
//                             *self.result = true;
//                         }
//                         break;
//                     }
//                 }
//                 else {
//                     break;
//                 }
//             }
//         }
//     }
// }
//
// struct StaticCheckTester<'a> {
//     result: &'a mut bool,
//     is_first_player: bool,
//     board: &'a Board,
//     king: &'a (Tile, Coordinate)
// }
//
// impl<'a> StaticCheckTester<'a> {
//     fn new(result: &'a mut bool,
//            is_first_player: bool,
//            board: &'a Board,
//            king: &'a (Tile, Coordinate)) -> StaticCheckTester<'a> {
//         StaticCheckTester { result, is_first_player, board, king }
//     }
//
//     fn test(&mut self, attacking_tile: Tile, transformations: &[(isize, isize)]) {
//         for transform in transformations {
//             let target_file = self.king.1.transform(transform.0);
//             let target_rank = self.king.2.transform(transform.1);
//
//             if let (Some(target_file), Some(target_rank)) = (target_file, target_rank) {
//                 let tile = self.board[(target_file, target_rank)];
//                 if tile == attacking_tile { *self.result = true; }
//             }
//         }
//     }
// }

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
        pawn_check {"8/8/8/8/3p4/4K3/8/8 w - - 0 1";true},
        rook_check_vertical {"8/8/8/8/8/4K3/8/4r3 w - - 0 1";true},
        rook_check_horizontal {"8/8/8/8/8/1r2K3/8/8 w - - 0 1";true},
        rook_no_check_if_blocked {"8/8/8/8/8/1r1RK3/8/8 w - - 0 1";false},
        bishop_check_upleft {"8/8/8/2b5/8/4K3/8/8 w - - 0 1";true},
        bishop_check_upright {"8/8/8/6b1/8/4K3/8/8 w - - 0 1";true},
        bishop_check_bottomleft {"8/8/8/8/8/4K3/8/2b5 w - - 0 1";true},
        bishop_check_bottomright {"8/8/8/8/8/4K3/8/6b1 w - - 0 1";true},
        bishop_no_check_if_blocked {"8/8/8/8/8/4K3/5B2/6b1 w - - 0 1";false},

        queen_check_vertical {"8/8/8/8/8/4K3/8/4q3 w - - 0 1";true},
        queen_check_horizontal {"8/8/8/8/8/1q2K3/8/8 w - - 0 1";true},
        queen_check_upleft {"8/8/8/2q5/8/4K3/8/8 w - - 0 1";true},
        queen_check_upright {"8/8/8/6q1/8/4K3/8/8 w - - 0 1";true},
        queen_check_bottomleft {"8/8/8/8/8/4K3/8/2q5 w - - 0 1";true},
        queen_check_bottomright {"8/8/8/8/8/4K3/8/6q1 w - - 0 1";true}
    }

    #[test]
    fn pawn_checks_in_opposite_direction_during_second_players_turn() {
        let game_state = GameState::from_fen("8/8/8/8/8/4k3/3P4/8 b - - 0 1");

        let result = is_check(false, &game_state);

        assert_eq!(true, result);
    }
}
