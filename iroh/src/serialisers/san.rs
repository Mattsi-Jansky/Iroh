use crate::state::coordinates::Coordinate;
use crate::state::tile::{Tile};

pub fn generate_san(tile: Tile, coordinate: Coordinate) -> String {
    format!("{}{}",
            to_piece_identifier(tile),
            coordinate)
}

pub fn generate_attack_san(tile: Tile, to: Coordinate) -> String {
    format!("{}x{}",
            to_piece_identifier(tile),
            to)
}

pub fn generate_pawn_san(target: Coordinate) -> String {
    format!("{}", target)
}

pub fn generate_pawn_attack_san(start: Coordinate, target: Coordinate) -> String{
    format!("{}x{}", start.file(), target)
}

pub fn generate_pawn_promotion_san(to: Coordinate, promote_to: Tile) -> String {
    format!("{}={}",
            to.file(),
            to_piece_identifier(promote_to)
    )
}

pub fn generate_castling_san(is_kingside: bool) -> String {
    if is_kingside { String::from("O-O") }
    else { String::from("O-O-O") }
}

fn to_piece_identifier(tile: Tile) -> char {
    match tile {
        Tile::FIRST_PAWN | Tile::SECOND_PAWN => panic!("Pawns should not be using to_piece_identifier"),
        Tile::FIRST_BISHOP | Tile::SECOND_BISHOP => 'B',
        Tile::FIRST_KNIGHT | Tile::SECOND_KNIGHT => 'N',
        Tile::FIRST_ROOK | Tile::SECOND_ROOK => 'R',
        Tile::FIRST_KING | Tile::SECOND_KING => 'K',
        Tile::FIRST_QUEEN | Tile::SECOND_QUEEN => 'Q',
        _ => { panic!("This should never happen - piece is not a valid recognised chesspiece") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_san() {
        let result = generate_san(Tile::FIRST_KNIGHT, Coordinate::C4);

        assert_eq!("Nc4", result);
    }

    #[test]
    fn should_generate_attack_san() {
        let result = generate_attack_san(Tile::FIRST_ROOK, Coordinate::C4);

        assert_eq!("Rxc4", result);
    }

    #[test]
    fn given_pawn_should_generate_san() {
        let result = generate_pawn_san(Coordinate::C4);

        assert_eq!("c4", result);
    }

    #[test]
    fn given_pawn_attack_should_generate_san() {
        let result = generate_pawn_attack_san(Coordinate::C3, Coordinate::D4);

        assert_eq!("cxd4", result);
    }

    #[test]
    fn given_kingside_castling_move_should_generate_san() {
        let result = generate_castling_san(true);

        assert_eq!(result, "O-O");
    }
}
