use crate::moves::Move;
use crate::piece::PieceType;

pub fn parse_san(san: &str) -> Move {
    let mut chars = san.chars();
    let column = (chars.next().unwrap() as u32) as usize - 97;
    let row = chars.next().unwrap().to_digit(10).unwrap() as usize - 1;

    Move::PawnMove { 0: column, 1: (column,row) }
}

pub fn generate_san(piece_type: PieceType, file: usize, rank: usize) -> String {
    String::from(format!("{}{}{}",
                         to_piece_identifier(piece_type),
                         to_alpha_file(file),
                         rank+1)
    )
}

pub fn generate_pawn_san(starting_rank: usize, file: usize, rank: usize) -> String {
    String::from(format!("{}{}", to_alpha_file(file), rank+1))
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
    fn parse_pawn_move() {
        let result = parse_san("e4");

        assert_eq!(Move::PawnMove(4, (4,3)), result);
    }

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
