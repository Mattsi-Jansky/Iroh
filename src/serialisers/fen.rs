use crate::piece::{Piece, PieceType};

pub fn parse_fen(fen: &str, callback: &mut dyn FnMut((usize, usize), Option<Piece>)) {
    let mut row = 7;
    let mut column = 0;

    for char in fen[..fen.len() - 13].chars() {
        if char.eq(&'/') {
            row -= 1;
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
        let is_owned_by_first_player = char.is_uppercase();
        callback((column, row),
                 piece_type.map(
                     |piece_type| Piece::new(is_owned_by_first_player, piece_type)
                 ));

        column += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fen_from_top_of_board_not_bottom() {
        let mut result: Vec<(usize,usize)> = vec![];
        let fen_that_forces_odd_numbered_rank_piece = "8/8/8/4n3/8/8/8/8 w KQkq - 0 1";

        parse_fen(fen_that_forces_odd_numbered_rank_piece, &mut |(column,row),piece| {
            result.push((column,row));
        });

        assert_eq!(1, result.len());
        assert_eq!(&(4,4), result.get(0).unwrap());
    }

    #[test]
    fn uppercase_is_first_player() {
        let mut result: Vec<Piece> = vec![];
        let fen_with_uppercase_king = "4K3/8/8/8/8/8/8/8 w KQkq - 0 1";

        parse_fen(fen_with_uppercase_king, &mut |(column,row),piece| {
            result.push(piece.unwrap());
        });

        assert_eq!(1, result.len());
        assert_eq!(true, result.get(0).unwrap().is_owned_by_first_player);
    }

    #[test]
    fn lowercase_is_second_player() {
        let mut result: Vec<Piece> = vec![];
        let fen_with_uppercase_king = "4K3/8/8/8/8/8/8/8 w KQkq - 0 1";

        parse_fen(fen_with_uppercase_king, &mut |(column,row),piece| {
            result.push(piece.unwrap());
        });

        assert_eq!(1, result.len());
        assert_eq!(true, result.get(0).unwrap().is_owned_by_first_player);
    }

}