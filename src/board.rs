use std::ops::{Index, IndexMut};
use crate::piece::ChessPiece;

pub struct Board {
    state: [Option<ChessPiece>; 8*8]
}

impl Board {
    fn blank() -> Board {
        Board {
            state: [
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
                None,None,None,None,None,None,None,None,
            ]
        }
    }
}

impl Index<(usize,usize)> for Board {
    type Output = Option<ChessPiece>;
    fn index(&self, s: (usize,usize)) -> &Option<ChessPiece> {
        return &self.state[s.0 * s.1];
    }
}

impl IndexMut<(usize,usize)> for Board {
    fn index_mut(&mut self, s: (usize,usize)) -> &mut Option<ChessPiece> {
        return &mut self.state[s.0 * s.1];
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::*;

    use crate::piece::ChessPieceType;
    use super::*;

    #[test]
    fn given_empty_tile_try_get_value_via_index() {
        let board = Board::blank();

        let result = board[(0,0)];

        assert!(result.is_none())
    }

    #[test]
    fn insert_piece_into_board_via_index() {
        let mut board = Board::blank();

        board[(0,0)] = Some(ChessPiece::new(true, ChessPieceType::King));
        let result = board[(0,0)];

        assert!(!result.is_none());
        let result = result.unwrap();
        assert_that!(&result.piece_type, eq(ChessPieceType::King));
    }

    #[test]
    fn index_outermost_corner_of_board() {
        let mut board = Board::blank();

        board[(7,7)] = Some(ChessPiece::new(true, ChessPieceType::King));
        let result = board[(7,7)];

        assert!(!result.is_none());
        let result = result.unwrap();
        assert_that!(&result.piece_type, eq(ChessPieceType::King));
    }
}
