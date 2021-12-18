use crate::state::coordinates::{File, Rank};
use crate::state::piece::PieceType;

pub fn generate_san(piece_type: PieceType, file: File, rank: Rank) -> String {
    format!("{}{}{}",
        to_piece_identifier(piece_type),
        to_alpha_file(file),
        rank+1)
}

pub fn generate_attack_san(piece_type: PieceType, file: File, rank: Rank) -> String {
    format!("{}x{}{}",
            to_piece_identifier(piece_type),
            to_alpha_file(file),
            rank+1)
}

pub fn generate_pawn_san(_starting_file: File,target_rank: Rank) -> String {
    format!("{}{}", to_alpha_file(_starting_file), target_rank+1)
}

pub fn generate_pawn_attack_san(_starting_file: File,target_file: File, target_rank: Rank) -> String{
    format!("{}x{}{}", to_alpha_file(_starting_file), to_alpha_file(target_file), target_rank+1)
}

pub fn generate_pawn_promotion_san(file: File, is_owned_by_first_player: bool, piece_type: PieceType) -> String {
    format!("{}{}={}",
            to_alpha_file(file),
            if is_owned_by_first_player {8} else {1},
            to_piece_identifier(piece_type)
    )
}

pub fn generate_castling_san(is_kingside: bool) -> String {
    if is_kingside { String::from("O-O") }
    else { String::from("O-O-O") }
}

fn to_alpha_file(file: File) -> char {
    (file + 97).into()
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
