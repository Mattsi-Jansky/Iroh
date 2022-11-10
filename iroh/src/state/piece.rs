#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PieceType {}

impl PieceType {
    pub const SECOND_PAWN: Piece = -1;
    pub const SECOND_BISHOP: Piece = -2;
    pub const SECOND_KNIGHT: Piece = -3;
    pub const SECOND_ROOK: Piece = -4;
    pub const SECOND_KING: Piece = -5;
    pub const SECOND_QUEEN: Piece = -6;
    pub const NONE: Piece = 0;
    pub const FIRST_PAWN: Piece = 1;
    pub const FIRST_BISHOP: Piece = 2;
    pub const FIRST_KNIGHT: Piece = 3;
    pub const FIRST_ROOK: Piece = 4;
    pub const FIRST_KING: Piece = 5;
    pub const FIRST_QUEEN: Piece = 6;
}

pub type Piece = i8;
