use crate::state::coordinates::{File, Rank};
use crate::state::piece::PieceType;
use crate::serialisers::san::{generate_attack_san, generate_castling_san, generate_pawn_attack_san, generate_pawn_promotion_san, generate_pawn_san, generate_san};

pub const KNIGHT_STATIC_TRANSFORMS: [(isize,isize);8] = [(1, 2), (2, 1), (-1, -2), (-2, -1), (1, -2), (2, -1), (-1, 2), (-2, 1)];
pub const KING_STATIC_TRANSFORMS : [(isize,isize);8] = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)];
pub const STRAIGHT_DYNAMIC_TRANSFORMS: [(isize, isize);4] = [(1, 0),(0, 1),(-1, 0),(0, -1)];
pub const DIAGONAL_DYNAMIC_TRANSFORMS: [(isize, isize);4] = [(1, 1),(1, -1),(-1, 1),(-1, -1)];

pub mod move_generation;
pub mod resolve_move;
mod pawn_moves;
mod static_moves;
mod dynamic_moves;
mod castling_moves;

#[derive(PartialEq, Debug, Clone)]
pub enum Move {
    RegularMove((File, Rank), (File, Rank), PieceType),
    AttackMove((File, Rank), (File, Rank), PieceType),
    PawnMove((File,Rank),Rank),
    PawnAttackMove(File,(File,Rank)),
    PawnPromotion(File,bool,PieceType),
    Castle(bool)
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
