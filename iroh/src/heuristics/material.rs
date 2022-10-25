use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::coordinates::{File, Rank};
use crate::state::GameState;
use crate::state::piece::PieceType;

pub struct MaterialHeuristic {}

impl Heuristic for MaterialHeuristic {
    fn evaluate(&self, state: &GameState, is_first_player: bool) -> i32 {
        let mut result = 0;

        result += state.board.get_all_pieces_belonging_to_player(is_first_player)
            .into_iter().map(|p| material_for(p)).sum::<i32>();
        result -= state.board.get_all_pieces_belonging_to_player(!is_first_player)
            .into_iter().map(|p| material_for(p)).sum::<i32>();

        result
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Material
    }
}

fn material_for(piece: (PieceType, File, Rank)) -> i32 {
    match piece.0 {
        PieceType::Pawn => { 1 }
        PieceType::Bishop => { 3 }
        PieceType::Knight => { 3 }
        PieceType::Rook => { 5 }
        PieceType::King => { 0 }
        PieceType::Queen => { 9 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_is_worth_one() {
        let state = GameState::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, true);

        assert_eq!(1, result);
    }

    #[test]
    fn given_multiple_pawns_count_all() {
        let state = GameState::from_fen("8/8/8/3PP3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, true);

        assert_eq!(2, result);
    }

    #[test]
    fn knight_and_bishop_worth_three_each() {
        let state= GameState::from_fen("8/8/8/3NB3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, true);

        assert_eq!(6, result);
    }

    #[test]
    fn rook_is_worth_five() {
        let state= GameState::from_fen("8/8/8/3R4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, true);

        assert_eq!(5, result);
    }

    #[test]
    fn queen_is_worth_nine() {
        let state= GameState::from_fen("8/8/8/3Q4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, true);

        assert_eq!(9, result);
    }

    #[test]
    fn minus_enemy_material_from_own_material() {
        let state= GameState::from_fen("8/8/8/3Qq3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(&state, true);

        assert_eq!(0, result);
    }
}
