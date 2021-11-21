use crate::coordinates::{File, Rank};
use crate::piece::PieceType;

pub fn generate_san(piece_type: PieceType, file: File, rank: Rank) -> String {
    format!("{}{}{}",
        to_piece_identifier(piece_type),
        to_alpha_file(file),
        rank+1)
}

pub fn generate_pawn_san(_starting_file: File, file: File, rank: Rank) -> String {
    format!("{}{}", to_alpha_file(file), rank+1)
}

fn to_alpha_file(file: File) -> char {
    *(file + 97) as u8 as char
}

fn to_piece_identifier(piece_type: PieceType) -> char {
    match piece_type {
        PieceType::Pawn => panic!("Pawns should not be using to_piece_identifier"),
        PieceType::Bishop => 'B',
        PieceType::Knight => 'N',
        PieceType::Rook => 'R',
        PieceType::King => 'K',
        PieceType::Queen => 'Q'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_san() {
        let result = generate_san(PieceType::Knight,File::new(2),Rank::new(3));

        assert_eq!("Nc4", result);
    }

    #[test]
    fn given_pawn_should_generate_san() {
        let result = generate_pawn_san(File::new(3),File::new(2),Rank::new(3));

        assert_eq!("c4", result);
    }
}
