#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChessPieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ChessPiece {
    pub is_owned_by_first_player: bool,
    pub piece_type: ChessPieceType,
}

impl ChessPiece {
    pub fn new(is_owned_by_first_player: bool, piece_type: ChessPieceType) -> ChessPiece {
        ChessPiece {is_owned_by_first_player, piece_type}
    }
}
