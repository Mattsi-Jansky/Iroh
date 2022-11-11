use crate::heuristics::{Heuristic, HeuristicType};
use crate::heuristics::cache::HeuristicsCache;
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::Piece;

pub struct MaterialHeuristic {}

impl Heuristic for MaterialHeuristic {
    fn evaluate(&self, state: &GameState, _: &HeuristicsCache) -> i32 {
        let mut result = 0;

        result += state.board.get_all_pieces_belonging_to_player(true)
            .into_iter().map(|p| p.0)
            .map(material_for).sum::<i32>();
        result -= state.board.get_all_pieces_belonging_to_player(false)
            .into_iter().map(|p| p.0)
            .map(material_for).sum::<i32>();

        result
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Material
    }
}

fn material_for(piece: Piece) -> i32 {
    match piece {
        Piece::FIRST_PAWN | Piece::SECOND_PAWN => { 1 }
        Piece::FIRST_BISHOP | Piece::SECOND_BISHOP => { 3 }
        Piece::FIRST_KNIGHT | Piece::SECOND_KNIGHT => { 3 }
        Piece::FIRST_ROOK | Piece::SECOND_ROOK => { 5 }
        Piece::FIRST_KING | Piece::SECOND_KING => { 0 }
        Piece::FIRST_QUEEN | Piece::SECOND_QUEEN => { 9 }
        Piece::NONE => { panic!("Cannot generate material for an empty tile") }
        _ => { panic!("This should never happen - piece is not a valid recognised chesspiece") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_is_worth_one() {
        let state = GameState::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(1, result);
    }

    #[test]
    fn given_multiple_pawns_count_all() {
        let state = GameState::from_fen("8/8/8/3PP3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(2, result);
    }

    #[test]
    fn knight_and_bishop_worth_three_each() {
        let state= GameState::from_fen("8/8/8/3NB3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(6, result);
    }

    #[test]
    fn rook_is_worth_five() {
        let state= GameState::from_fen("8/8/8/3R4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(5, result);
    }

    #[test]
    fn queen_is_worth_nine() {
        let state= GameState::from_fen("8/8/8/3Q4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(9, result);
    }

    #[test]
    fn minus_enemy_material_from_own_material() {
        let state= GameState::from_fen("8/8/8/3Qq3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, &HeuristicsCache::from(&state));

        assert_eq!(0, result);
    }
}
