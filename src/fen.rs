use crate::piece::{ChessPiece, ChessPieceType};

pub fn parse_fen(fen: &str, callback: &mut dyn FnMut((usize, usize), Option<ChessPiece>)) {
    let mut row = 0;
    let mut column = 0;

    for char in fen[..fen.len() - 13].chars() {
        if char.eq(&'/') {
            row += 1;
            column = 0;
            continue;
        }
        if char.is_digit(10) {
            column += char as usize - 0x30;
            continue;
        }

        let piece_type = match char {
            'r' | 'R' => Some(ChessPieceType::Rook),
            'n' | 'N' => Some(ChessPieceType::Knight),
            'b' | 'B' => Some(ChessPieceType::Bishop),
            'q' | 'Q' => Some(ChessPieceType::Queen),
            'k' | 'K' => Some(ChessPieceType::King),
            'p' | 'P' => Some(ChessPieceType::Pawn),
            _ => None
        };
        let is_owned_by_first_player = !char.is_uppercase();
        callback((column, row),
                 piece_type.map(
                     |piece_type| ChessPiece::new(is_owned_by_first_player,piece_type)
                 ));

        column += 1;
    }
}
