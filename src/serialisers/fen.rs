use crate::piece::{Piece, PieceType};

pub fn parse_fen(fen: &str, callback: &mut dyn FnMut((usize, usize), Option<Piece>)) {
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
            'r' | 'R' => Some(PieceType::Rook),
            'n' | 'N' => Some(PieceType::Knight),
            'b' | 'B' => Some(PieceType::Bishop),
            'q' | 'Q' => Some(PieceType::Queen),
            'k' | 'K' => Some(PieceType::King),
            'p' | 'P' => Some(PieceType::Pawn),
            _ => None
        };
        let is_owned_by_first_player = !char.is_uppercase();
        callback((column, row),
                 piece_type.map(
                     |piece_type| Piece::new(is_owned_by_first_player, piece_type)
                 ));

        column += 1;
    }
}
