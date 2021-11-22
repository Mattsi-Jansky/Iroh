use crate::piece::PieceType;

#[derive(Debug, Clone, PartialEq)]
pub struct CapturedPieces {
    pub first_player: Vec<PieceType>,
    pub second_player: Vec<PieceType>
}

impl CapturedPieces {
    pub fn new() -> CapturedPieces { CapturedPieces {first_player: vec![], second_player: vec![]} }
}
