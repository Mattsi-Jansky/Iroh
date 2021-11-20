use crate::piece::PieceType;

pub fn generate_san(piece_type: PieceType, file: usize, rank: usize) -> String {
    format!("{}{}{}",
        to_piece_identifier(piece_type),
        to_alpha_file(file),
        rank+1)
}

pub fn generate_pawn_san(_starting_rank: usize, file: usize, rank: usize) -> String {
    format!("{}{}", to_alpha_file(file), rank+1)
}

fn to_alpha_file(file: usize) -> char {
    (file + 97) as u8 as char
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
        let result = generate_san(PieceType::Knight,2,3);

        assert_eq!("Nc4", result);
    }

    #[test]
    fn given_pawn_should_generate_san() {
        let result = generate_pawn_san(3,2,3);

        assert_eq!("c4", result);
    }
}
