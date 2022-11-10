use derive_more::{Deref, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, Display, Hash)]
pub struct Piece(i8);

impl Piece {
    pub const SECOND_PAWN: Piece = Piece(-1);
    pub const SECOND_BISHOP: Piece = Piece(-2);
    pub const SECOND_KNIGHT: Piece = Piece(-3);
    pub const SECOND_ROOK: Piece = Piece(-4);
    pub const SECOND_KING: Piece = Piece(-5);
    pub const SECOND_QUEEN: Piece = Piece(-6);
    pub const NONE: Piece = Piece(0);
    pub const FIRST_PAWN: Piece = Piece(1);
    pub const FIRST_BISHOP: Piece = Piece(2);
    pub const FIRST_KNIGHT: Piece = Piece(3);
    pub const FIRST_ROOK: Piece = Piece(4);
    pub const FIRST_KING: Piece = Piece(5);
    pub const FIRST_QUEEN: Piece = Piece(6);

    pub fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    pub fn is_owned_by_first_player(&self) -> bool {
        self.0 > 0
    }

    pub fn inverted_ownership(&self) -> Piece {
        Piece(-self.0)
    }
}
