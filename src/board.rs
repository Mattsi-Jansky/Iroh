use std::ops::{Index, IndexMut};
use crate::coordinates::{File, Rank};
use crate::serialisers::fen::parse_fen;
use crate::piece::{Piece, PieceType};

const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    state: [Option<Piece>; 8*8]
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

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::blank();

        parse_fen(fen, &mut |(file, rank), piece| {
            board.state[*file + *(rank * 8)] = piece;
        });

        board
    }

    pub(crate) fn get_all_pieces_belonging_to_player(&self, is_owned_by_first_player: bool) -> Vec<(PieceType, File, Rank)> {
        let mut result = vec![];

        for (index, piece_or_none) in self.state.iter().enumerate() {
            if let Some(piece) = piece_or_none {
                if piece.is_owned_by_first_player == is_owned_by_first_player {
                    result.push((piece.piece_type, File::new(index % 8), Rank::new(index / 8)));
                }
            }
        }

        result
    }
}

impl Index<(File,Rank)> for Board {
    type Output = Option<Piece>;
    fn index(&self, s: (File,Rank)) -> &Option<Piece> {
        &self.state[*s.0 + *(s.1 * 8)]
    }
}

impl IndexMut<(File,Rank)> for Board {
    fn index_mut(&mut self, s: (File,Rank)) -> &mut Option<Piece> {
        &mut self.state[*s.0 + *(s.1 * 8)]
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
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

        let result = board[(File::new(0),Rank::new(0))];

        assert!(result.is_none())
    }

    #[test]
    fn insert_piece_into_board_via_index() {
        let mut board = Board::blank();

        board[(File::new(0),Rank::new(0))] = Some(Piece::new(true, PieceType::King));
        let result = board[(File::new(0),Rank::new(0))];

        assert!(!result.is_none());
        let result = result.unwrap();
        assert_that!(&result.piece_type, eq(PieceType::King));
    }

    #[test]
    fn index_outermost_corner_of_board() {
        let mut board = Board::blank();

        board[(File::new(7),Rank::new(7))] = Some(Piece::new(true, PieceType::King));
        let result = board[(File::new(7),Rank::new(7))];

        assert!(!result.is_none());
        let result = result.unwrap();
        assert_that!(&result.piece_type, eq(PieceType::King));
    }

    #[test]
    fn create_board_from_fen_layout() {
        let result = Board::from_fen(STARTING_POSITION_FEN);

        assert_that!(&result[(File::new(0),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Rook)));
        assert_that!(&result[(File::new(1),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Knight)));
        assert_that!(&result[(File::new(2),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Bishop)));
        assert_that!(&result[(File::new(3),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Queen)));
        assert_that!(&result[(File::new(4),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::King)));
        assert_that!(&result[(File::new(5),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Bishop)));
        assert_that!(&result[(File::new(6),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Knight)));
        assert_that!(&result[(File::new(7),Rank::new(0))].unwrap(), eq(Piece::new(true, PieceType::Rook)));

        assert_that!(&result[(File::new(0),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(1),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(2),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(3),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(4),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(5),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(6),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));
        assert_that!(&result[(File::new(7),Rank::new(1))].unwrap(), eq(Piece::new(true, PieceType::Pawn)));

        assert!(result[(File::new(0),Rank::new(2))].is_none());
        assert!(result[(File::new(1),Rank::new(2))].is_none());
        assert!(result[(File::new(2),Rank::new(2))].is_none());
        assert!(result[(File::new(3),Rank::new(2))].is_none());
        assert!(result[(File::new(4),Rank::new(2))].is_none());
        assert!(result[(File::new(5),Rank::new(2))].is_none());
        assert!(result[(File::new(6),Rank::new(2))].is_none());
        assert!(result[(File::new(7),Rank::new(2))].is_none());

        assert!(result[(File::new(0),Rank::new(3))].is_none());
        assert!(result[(File::new(1),Rank::new(3))].is_none());
        assert!(result[(File::new(2),Rank::new(3))].is_none());
        assert!(result[(File::new(3),Rank::new(3))].is_none());
        assert!(result[(File::new(4),Rank::new(3))].is_none());
        assert!(result[(File::new(5),Rank::new(3))].is_none());
        assert!(result[(File::new(6),Rank::new(3))].is_none());
        assert!(result[(File::new(7),Rank::new(3))].is_none());

        assert!(result[(File::new(0),Rank::new(4))].is_none());
        assert!(result[(File::new(1),Rank::new(4))].is_none());
        assert!(result[(File::new(2),Rank::new(4))].is_none());
        assert!(result[(File::new(3),Rank::new(4))].is_none());
        assert!(result[(File::new(4),Rank::new(4))].is_none());
        assert!(result[(File::new(5),Rank::new(4))].is_none());
        assert!(result[(File::new(6),Rank::new(4))].is_none());
        assert!(result[(File::new(7),Rank::new(4))].is_none());

        assert!(result[(File::new(0),Rank::new(5))].is_none());
        assert!(result[(File::new(1),Rank::new(5))].is_none());
        assert!(result[(File::new(2),Rank::new(5))].is_none());
        assert!(result[(File::new(3),Rank::new(5))].is_none());
        assert!(result[(File::new(4),Rank::new(5))].is_none());
        assert!(result[(File::new(5),Rank::new(5))].is_none());
        assert!(result[(File::new(6),Rank::new(5))].is_none());
        assert!(result[(File::new(7),Rank::new(5))].is_none());

        assert_that!(&result[(File::new(0),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(1),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(2),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(3),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(4),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(5),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(6),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));
        assert_that!(&result[(File::new(7),Rank::new(6))].unwrap(), eq(Piece::new(false, PieceType::Pawn)));

        assert_that!(&result[(File::new(0),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Rook)));
        assert_that!(&result[(File::new(1),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Knight)));
        assert_that!(&result[(File::new(2),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Bishop)));
        assert_that!(&result[(File::new(3),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Queen)));
        assert_that!(&result[(File::new(4),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::King)));
        assert_that!(&result[(File::new(5),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Bishop)));
        assert_that!(&result[(File::new(6),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Knight)));
        assert_that!(&result[(File::new(7),Rank::new(7))].unwrap(), eq(Piece::new(false, PieceType::Rook)));
    }

    #[test]
    fn get_all_pieces_of_type_and_ownership() {
        let board = Board::new();

        let result = board.get_all_pieces_belonging_to_player(true);

        assert_that!(&result, contains_in_any_order(vec![
            (PieceType::Pawn,File::new(0),Rank::new(1)),
            (PieceType::Pawn,File::new(1),Rank::new(1)),
            (PieceType::Pawn,File::new(2),Rank::new(1)),
            (PieceType::Pawn,File::new(3),Rank::new(1)),
            (PieceType::Pawn,File::new(4),Rank::new(1)),
            (PieceType::Pawn,File::new(5),Rank::new(1)),
            (PieceType::Pawn,File::new(6),Rank::new(1)),
            (PieceType::Pawn,File::new(7),Rank::new(1)),
            (PieceType::Rook,File::new(0),Rank::new(0)),
            (PieceType::Rook,File::new(7),Rank::new(0)),
            (PieceType::Knight,File::new(1),Rank::new(0)),
            (PieceType::Knight,File::new(6),Rank::new(0)),
            (PieceType::Bishop,File::new(5),Rank::new(0)),
            (PieceType::Bishop,File::new(2),Rank::new(0)),
            (PieceType::Queen,File::new(3),Rank::new(0)),
            (PieceType::King,File::new(4),Rank::new(0)),
        ]));
    }
}
