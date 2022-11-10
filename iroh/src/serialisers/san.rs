use crate::state::coordinates::{File, Rank};
use crate::state::piece::{Piece, PieceType};

pub fn generate_san(piece: Piece, file: File, rank: Rank) -> String {
    format!("{}{}{}",
            to_piece_identifier(piece),
            to_alpha_file(file),
            rank+1)
}

pub fn generate_attack_san(piece: Piece, file: File, rank: Rank) -> String {
    format!("{}x{}{}",
            to_piece_identifier(piece),
            to_alpha_file(file),
            rank+1)
}

pub fn generate_pawn_san(_starting_file: File,target_rank: Rank) -> String {
    format!("{}{}", to_alpha_file(_starting_file), target_rank+1)
}

pub fn generate_pawn_attack_san(_starting_file: File,target_file: File, target_rank: Rank) -> String{
    format!("{}x{}{}", to_alpha_file(_starting_file), to_alpha_file(target_file), target_rank+1)
}

pub fn generate_pawn_promotion_san(file: File, is_owned_by_first_player: bool, piece: Piece) -> String {
    format!("{}{}={}",
            to_alpha_file(file),
            if is_owned_by_first_player {8} else {1},
            to_piece_identifier(piece)
    )
}

pub fn generate_castling_san(is_kingside: bool) -> String {
    if is_kingside { String::from("O-O") }
    else { String::from("O-O-O") }
}

fn to_alpha_file(file: File) -> char {
    (file + 97).into()
}

fn to_piece_identifier(piece: Piece) -> char {
    match piece {
        PieceType::FIRST_PAWN | PieceType::SECOND_PAWN | 1 => panic!("Pawns should not be using to_piece_identifier"),
        PieceType::FIRST_BISHOP | PieceType::SECOND_BISHOP => 'B',
        PieceType::FIRST_KNIGHT | PieceType::SECOND_KNIGHT => 'N',
        PieceType::FIRST_ROOK | PieceType::SECOND_ROOK => 'R',
        PieceType::FIRST_KING | PieceType::SECOND_KING => 'K',
        PieceType::FIRST_QUEEN | PieceType::SECOND_QUEEN => 'Q'
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
    fn should_generate_attack_san() {
        let result = generate_attack_san(PieceType::Rook, File::new(2), Rank::new(3));

        assert_eq!("Rxc4", result);
    }

    #[test]
    fn given_pawn_should_generate_san() {
        let result = generate_pawn_san(File::new(2),Rank::new(3));

        assert_eq!("c4", result);
    }

    #[test]
    fn given_pawn_attack_should_generate_san() {
        let result = generate_pawn_attack_san(File::new(2), File::new(3), Rank::new(3));

        assert_eq!("cxd4", result);
    }

    #[test]
    fn given_kingside_castling_move_should_generate_san() {
        let result = generate_castling_san(true);

        assert_eq!(result, "O-O");
    }
}
