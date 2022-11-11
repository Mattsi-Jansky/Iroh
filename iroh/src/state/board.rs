use std::ops::{Index, IndexMut};
use crate::state::coordinates::{File, Rank};
use crate::state::piece::{Piece};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    state: [Piece; 8*8]
}

impl Board {
    pub fn blank() -> Board {
        Board {
            state: [
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
                Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,Piece::NONE,
            ]
        }
    }

    pub(crate) fn get_all_pieces_belonging_to_player(&self, is_owned_by_first_player: bool) -> Vec<(Piece, File, Rank)> {
        let mut result = vec![];

        for (index, tile) in self.state.iter().enumerate() {
            if tile != &Piece::NONE {
                if tile.is_occupied() == is_owned_by_first_player {
                    result.push((tile.clone(), File::new(index % 8), Rank::new(index / 8)));
                }
            }
        }

        result
    }
}

impl Index<(File,Rank)> for Board {
    type Output = Piece;
    fn index(&self, s: (File,Rank)) -> &Piece {
        &self.state[*s.0 + *(s.1 * 8)]
    }
}

impl Index<(&File,&Rank)> for Board {
    type Output = Piece;
    fn index(&self, s: (&File,&Rank)) -> &Piece {
        &self.state[*(*s.0 + *(*s.1 * 8))]
    }
}

impl IndexMut<(File,Rank)> for Board {
    fn index_mut(&mut self, s: (File,Rank)) -> &mut Piece {
        &mut self.state[*s.0 + *(s.1 * 8)]
    }
}

impl IndexMut<(&File,&Rank)> for Board {
    fn index_mut(&mut self, s: (&File,&Rank)) -> &mut Piece {
        &mut self.state[*(*s.0 + *(*s.1 * 8))]
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

    use crate::state::GameState;
    use super::*;

    #[test]
    fn given_empty_tile_try_get_value_via_index() {
        let board = Board::blank();

        let result = board[(File::new(0),Rank::new(0))];

        assert_eq!(result, Piece::NONE)
    }

    #[test]
    fn insert_piece_into_board_via_index() {
        let mut board = Board::blank();

        board[(File::new(0),Rank::new(0))] = Piece::FIRST_KING;
        let result = board[(File::new(0),Rank::new(0))];

        assert_eq!(result, Piece::FIRST_KING);
    }

    #[test]
    fn index_outermost_corner_of_board() {
        let mut board = Board::blank();

        board[(File::new(7),Rank::new(7))] = Piece::FIRST_KING;
        let result = board[(File::new(7),Rank::new(7))];

        assert_eq!(result, Piece::FIRST_KING);
    }

    #[test]
    fn get_all_pieces_from_ownership() {
        let board = GameState::new().board;

        let result = board.get_all_pieces_belonging_to_player(true);

        assert_that!(&result, contains_in_any_order(vec![
            (Piece::FIRST_PAWN,File::new(0),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(1),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(2),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(3),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(4),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(5),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(6),Rank::new(1)),
            (Piece::FIRST_PAWN,File::new(7),Rank::new(1)),
            (Piece::FIRST_ROOK,File::new(0),Rank::new(0)),
            (Piece::FIRST_ROOK,File::new(7),Rank::new(0)),
            (Piece::FIRST_KNIGHT,File::new(1),Rank::new(0)),
            (Piece::FIRST_KNIGHT,File::new(6),Rank::new(0)),
            (Piece::FIRST_BISHOP,File::new(5),Rank::new(0)),
            (Piece::FIRST_BISHOP,File::new(2),Rank::new(0)),
            (Piece::FIRST_QUEEN,File::new(3),Rank::new(0)),
            (Piece::FIRST_KING,File::new(4),Rank::new(0)),
        ]));
    }
}
