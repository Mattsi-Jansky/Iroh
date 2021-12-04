use crate::state::coordinates::{File, Rank};
use crate::state::piece::PieceType;
use crate::serialisers::san::{generate_attack_san, generate_pawn_attack_san, generate_pawn_promotion_san, generate_pawn_san, generate_san};

pub mod move_generation;
pub mod resolve_move;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Move {
    RegularMove((File, Rank), (File, Rank), PieceType),
    AttackMove((File, Rank), (File, Rank), PieceType),
    PawnMove((File,Rank),Rank),
    PawnAttackMove(File,(File,Rank)),
    PawnPromotion(File,bool,PieceType)
}

impl Move {
    pub fn generate_san(&self) -> String
    {
        match *self {
            Move::RegularMove((_,_),(file,rank), piece_type) => {
                generate_san(piece_type,file,rank)
            },
            Move::PawnMove(from, target_rank) => {
                generate_pawn_san(from.0, target_rank)
            }
            Move::AttackMove((_,_),(file,rank), piece_type) => {
                generate_attack_san(piece_type,file,rank)
            }
            Move::PawnAttackMove(starting_file,(target_file, target_rank)) => {
                generate_pawn_attack_san(starting_file, target_file, target_rank)
            },
            Move::PawnPromotion(file, is_owned_by_first_player, piece_type) => {
                generate_pawn_promotion_san(file, is_owned_by_first_player, piece_type)
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
            (File::new(0),Rank::new(0)),
            (File::new(2),Rank::new(3)),
            PieceType::Knight);

        let result = test_move.generate_san();

        assert_eq!("Nc4", result);
    }

    #[test]
    fn given_pawn_move_should_generate_san() {
        let test_move = Move::PawnMove((File::new(2), Rank::new(2)),
                                       Rank::new(3));

        let result = test_move.generate_san();

        assert_eq!("c4", result);
    }
}
