use std::ops::{Index, IndexMut};
use crate::state::coordinates::Coordinate;
use crate::state::tile::{Tile};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    state: [Tile; 8*8]
}

impl Board {
    pub fn blank() -> Board {
        Board {
            state: [
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
                Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY, Tile::EMPTY,
            ]
        }
    }

    pub(crate) fn get_all_pieces_belonging_to_player(&self, is_owned_by_first_player: bool) -> Vec<(Tile, Coordinate)> {
        let mut result = vec![];

        for (index, tile) in self.state.iter().enumerate() {
            if tile.is_occupied() {
                if tile.is_owned_by_first_player() == is_owned_by_first_player {
                    result.push((tile.clone(), Coordinate::from(index)));
                }
            }
        }

        result
    }
}

impl Index<Coordinate> for Board {
    type Output = Tile;
    fn index(&self, s: Coordinate) -> &Tile {
        &self.state[s.as_usize()]
    }
}

impl Index<&Coordinate> for Board {
    type Output = Tile;
    fn index(&self, s: &Coordinate) -> &Tile {
        &self.state[s.as_usize()]
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, s: Coordinate) -> &mut Tile {
        &mut self.state[s.as_usize()]
    }
}

impl IndexMut<&Coordinate> for Board {
    fn index_mut(&mut self, s: &Coordinate) -> &mut Tile {
        &mut self.state[s.as_usize()]
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

        let result = board[Coordinate::A1];

        assert_eq!(result, Tile::EMPTY)
    }

    #[test]
    fn insert_piece_into_board_via_index() {
        let mut board = Board::blank();

        board[Coordinate::A1] = Tile::FIRST_KING;
        let result = board[Coordinate::A1];

        assert_eq!(result, Tile::FIRST_KING);
    }

    #[test]
    fn index_outermost_corner_of_board() {
        let mut board = Board::blank();

        board[Coordinate::H8] = Tile::FIRST_KING;
        let result = board[Coordinate::H8];

        assert_eq!(result, Tile::FIRST_KING);
    }

    #[test]
    fn get_all_pieces_from_ownership() {
        let board = GameState::new().board;

        let result = board.get_all_pieces_belonging_to_player(true);

        assert_that!(&result, contains_in_any_order(vec![
            (Tile::FIRST_PAWN,Coordinate::A2),
            (Tile::FIRST_PAWN,Coordinate::B2),
            (Tile::FIRST_PAWN,Coordinate::C2),
            (Tile::FIRST_PAWN,Coordinate::D2),
            (Tile::FIRST_PAWN,Coordinate::E2),
            (Tile::FIRST_PAWN,Coordinate::F2),
            (Tile::FIRST_PAWN,Coordinate::G2),
            (Tile::FIRST_PAWN,Coordinate::H2),
            (Tile::FIRST_ROOK,Coordinate::A1),
            (Tile::FIRST_ROOK,Coordinate::H1),
            (Tile::FIRST_KNIGHT,Coordinate::B1),
            (Tile::FIRST_KNIGHT,Coordinate::G1),
            (Tile::FIRST_BISHOP,Coordinate::F1),
            (Tile::FIRST_BISHOP,Coordinate::C1),
            (Tile::FIRST_QUEEN,Coordinate::D1),
            (Tile::FIRST_KING,Coordinate::E1),
        ]));
    }
}
