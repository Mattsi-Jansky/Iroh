use std::ops::{Index, IndexMut};
use crate::fen::parse_fen;
use crate::piece::{ChessPiece, ChessPieceType};

const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    state: [Option<ChessPiece>; 8*8]
}

impl Board {
    pub fn new() -> Board {
        Board::from_fen(STARTING_POSITION_FEN)
    }

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

    fn from_fen(fen: &str) -> Board {
        let mut board = Board::blank();

        parse_fen(fen, &mut |(column,row),piece| {
            board.state[column + (row * 8)] = piece;
        });

        board
    }

    pub(crate) fn get_all(&self, piece_type: ChessPieceType, is_owned_by_first_player: bool) -> Vec<(ChessPieceType,usize,usize)> {
        let mut result = vec![];

        for (index, piece_or_none) in self.state.iter().enumerate() {
            if let Some(piece) = piece_or_none {
                if piece.is_owned_by_first_player == is_owned_by_first_player
                    && piece.piece_type == piece_type {
                    result.push((piece.piece_type, index % 8, index / 8));
                }
            }
        }

        result
    }
}

impl Index<(usize,usize)> for Board {
    type Output = Option<ChessPiece>;
    fn index(&self, s: (usize,usize)) -> &Option<ChessPiece> {
        &self.state[s.0 + (s.1 * 8)]
    }
}

impl IndexMut<(usize,usize)> for Board {
    fn index_mut(&mut self, s: (usize,usize)) -> &mut Option<ChessPiece> {
        &mut self.state[s.0 + (s.1 * 8)]
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

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

    #[test]
    fn create_board_from_fen_layout() {
        let result = Board::from_fen(STARTING_POSITION_FEN);

        assert_that!(&result[(0,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Rook)));
        assert_that!(&result[(1,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Knight)));
        assert_that!(&result[(2,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Bishop)));
        assert_that!(&result[(3,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Queen)));
        assert_that!(&result[(4,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::King)));
        assert_that!(&result[(5,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Bishop)));
        assert_that!(&result[(6,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Knight)));
        assert_that!(&result[(7,0)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Rook)));

        assert_that!(&result[(0,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(1,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(2,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(3,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(4,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(5,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(6,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));
        assert_that!(&result[(7,1)].unwrap(), eq(ChessPiece::new(true, ChessPieceType::Pawn)));

        assert!(result[(0,2)].is_none());
        assert!(result[(1,2)].is_none());
        assert!(result[(2,2)].is_none());
        assert!(result[(3,2)].is_none());
        assert!(result[(4,2)].is_none());
        assert!(result[(5,2)].is_none());
        assert!(result[(6,2)].is_none());
        assert!(result[(7,2)].is_none());

        assert!(result[(0,3)].is_none());
        assert!(result[(1,3)].is_none());
        assert!(result[(2,3)].is_none());
        assert!(result[(3,3)].is_none());
        assert!(result[(4,3)].is_none());
        assert!(result[(5,3)].is_none());
        assert!(result[(6,3)].is_none());
        assert!(result[(7,3)].is_none());

        assert!(result[(0,4)].is_none());
        assert!(result[(1,4)].is_none());
        assert!(result[(2,4)].is_none());
        assert!(result[(3,4)].is_none());
        assert!(result[(4,4)].is_none());
        assert!(result[(5,4)].is_none());
        assert!(result[(6,4)].is_none());
        assert!(result[(7,4)].is_none());

        assert!(result[(0,5)].is_none());
        assert!(result[(1,5)].is_none());
        assert!(result[(2,5)].is_none());
        assert!(result[(3,5)].is_none());
        assert!(result[(4,5)].is_none());
        assert!(result[(5,5)].is_none());
        assert!(result[(6,5)].is_none());
        assert!(result[(7,5)].is_none());

        assert_that!(&result[(0,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(1,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(2,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(3,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(4,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(5,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(6,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));
        assert_that!(&result[(7,6)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Pawn)));

        assert_that!(&result[(0,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Rook)));
        assert_that!(&result[(1,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Knight)));
        assert_that!(&result[(2,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Bishop)));
        assert_that!(&result[(3,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Queen)));
        assert_that!(&result[(4,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::King)));
        assert_that!(&result[(5,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Bishop)));
        assert_that!(&result[(6,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Knight)));
        assert_that!(&result[(7,7)].unwrap(), eq(ChessPiece::new(false, ChessPieceType::Rook)));
    }

    #[test]
    fn get_all_pieces_of_type_and_ownership() {
        let board = Board::new();

        let result = board.get_all(ChessPieceType::Pawn, true);

        assert_that!(&result, contains_in_any_order(vec![
            (ChessPieceType::Pawn,0,1),
            (ChessPieceType::Pawn,1,1),
            (ChessPieceType::Pawn,2,1),
            (ChessPieceType::Pawn,3,1),
            (ChessPieceType::Pawn,4,1),
            (ChessPieceType::Pawn,5,1),
            (ChessPieceType::Pawn,6,1),
            (ChessPieceType::Pawn,7,1)
        ]));
    }
}