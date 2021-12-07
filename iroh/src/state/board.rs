use std::ops::{Index, IndexMut};
use crate::state::coordinates::{File, Rank};
use crate::state::piece::{Piece, PieceType};

const STARTING_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    state: [Option<Piece>; 8*8]
}

impl Board {
    pub fn blank() -> Board {
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
        Board::blank()
    }
}

#[cfg(test)]
mod tests {
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

    use crate::serialisers::fen::parse_fen;
    use crate::state::GameState;
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
    fn get_all_pieces_of_type_and_ownership() {
        let board = GameState::new().board;

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
