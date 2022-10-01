#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub is_owned_by_first_player: bool,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(is_owned_by_first_player: bool, piece_type: PieceType) -> Piece {
        Piece {is_owned_by_first_player, piece_type}
    }
}
