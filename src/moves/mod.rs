use crate::board::Board;
use crate::piece::PieceType;
use crate::serialisers::san::{generate_pawn_san, generate_san};

pub mod move_generation;
pub mod resolve_move;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Move {
    RegularMove((usize, usize), (usize, usize), PieceType),
    PawnMove(usize,(usize,usize))
}

impl Move {
    pub fn generate_san(&self) -> String
    {
        match *self {
            Move::RegularMove((_,_),(file,rank), piece_type) => {
                // String::from(format!("{}{}{}",))
                generate_san(piece_type,file,rank)
            },
            Move::PawnMove(starting_file,(file,rank)) => {
                generate_pawn_san(starting_file,file,rank)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_san() {
        let test_move = Move::RegularMove((0,0),(2,3), PieceType::Knight);

        let result = test_move.generate_san();

        assert_eq!("Nc4", result);
    }

    #[test]
    fn given_pawn_move_should_generate_san() {
        let test_move = Move::PawnMove(0,(2,3));

        let result = test_move.generate_san();

        assert_eq!("c4", result);
    }
}
