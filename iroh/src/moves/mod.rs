use std::fmt::{Display, Formatter};
use crate::state::tile::{Tile};
use crate::serialisers::san::{generate_attack_san, generate_castling_san, generate_pawn_attack_san, generate_pawn_promotion_san, generate_pawn_san, generate_san};
use crate::state::coordinates::Coordinate;

pub mod move_generation;
pub mod resolve_move;
pub(crate) mod coordinate_transformers;
mod pawn_moves;
mod static_moves;
mod dynamic_moves;
mod castling_moves;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Move {
    RegularMove(Coordinate, Coordinate, Tile),
    AttackMove(Coordinate, Coordinate, Tile),
    PawnMove(Coordinate,Coordinate),
    PawnAttackMove(Coordinate,Coordinate),
    PawnPromotion(Coordinate, Tile),
    Castle(bool)
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate_san())
    }
}

impl Move {
    pub fn generate_san(&self) -> String
    {
        match *self {
            Move::RegularMove(_, coordinate, piece_type) => {
                generate_san(piece_type, coordinate)
            }
            Move::PawnMove(_, to) => {
                generate_pawn_san(to)
            }
            Move::AttackMove(_, coordinate, piece_type) => {
                generate_attack_san(piece_type, coordinate)
            }
            Move::PawnAttackMove(from, to) => {
                generate_pawn_attack_san(from, to)
            }
            Move::PawnPromotion(file, piece) => {
                generate_pawn_promotion_san(file, piece)
            }
            Move::Castle(is_kingside) => {
                generate_castling_san(is_kingside)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_san() {
        let test_move = Move::RegularMove(
            Coordinate::B2,
            Coordinate::C4,
            Tile::FIRST_KNIGHT);

        let result = test_move.generate_san();

        assert_eq!("Nc4", result);
    }

    #[test]
    fn given_pawn_move_should_generate_san() {
        let test_move = Move::PawnMove(Coordinate::C3,
                                       Coordinate::C4);

        let result = test_move.generate_san();

        assert_eq!("c4", result);
    }
}
