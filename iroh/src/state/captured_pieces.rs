use crate::state::piece::PieceType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedPieces {
    pub first_player: Vec<PieceType>,
    pub second_player: Vec<PieceType>
}

impl CapturedPieces {
    pub fn new() -> CapturedPieces { CapturedPieces {first_player: vec![], second_player: vec![]} }
}

impl Default for CapturedPieces {
    fn default() -> Self {
        CapturedPieces::new()
    }
}
