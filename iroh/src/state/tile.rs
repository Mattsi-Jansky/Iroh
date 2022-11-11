use derive_more::{Deref, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, Display, Hash)]
pub struct Tile(i8);

impl Tile {
    pub const SECOND_PAWN: Tile = Tile(-1);
    pub const SECOND_BISHOP: Tile = Tile(-2);
    pub const SECOND_KNIGHT: Tile = Tile(-3);
    pub const SECOND_ROOK: Tile = Tile(-4);
    pub const SECOND_KING: Tile = Tile(-5);
    pub const SECOND_QUEEN: Tile = Tile(-6);
    pub const EMPTY: Tile = Tile(0);
    pub const FIRST_PAWN: Tile = Tile(1);
    pub const FIRST_BISHOP: Tile = Tile(2);
    pub const FIRST_KNIGHT: Tile = Tile(3);
    pub const FIRST_ROOK: Tile = Tile(4);
    pub const FIRST_KING: Tile = Tile(5);
    pub const FIRST_QUEEN: Tile = Tile(6);

    pub fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    pub fn is_owned_by_first_player(&self) -> bool {
        self.0 > 0
    }

    pub fn inverted_ownership(&self) -> Tile {
        Tile(-self.0)
    }
}
