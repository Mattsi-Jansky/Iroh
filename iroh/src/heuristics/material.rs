use crate::heuristics::cache::HeuristicsCache;
use crate::heuristics::{Heuristic, HeuristicType};
use crate::state::tile::Tile;
use crate::state::GameState;

pub struct MaterialHeuristic {}

impl Heuristic for MaterialHeuristic {
    fn evaluate(&self, state: &GameState, _: &HeuristicsCache) -> i32 {
        let mut result = 0;

        result += state
            .board
            .get_all_pieces_belonging_to_player(true)
            .into_iter()
            .map(|p| p.0)
            .map(material_for)
            .sum::<i32>();
        result -= state
            .board
            .get_all_pieces_belonging_to_player(false)
            .into_iter()
            .map(|p| p.0)
            .map(material_for)
            .sum::<i32>();

        result
    }

    fn get_type(&self) -> HeuristicType {
        HeuristicType::Material
    }
}

fn material_for(tile: Tile) -> i32 {
    match tile {
        Tile::FIRST_PAWN | Tile::SECOND_PAWN => 1,
        Tile::FIRST_BISHOP | Tile::SECOND_BISHOP => 3,
        Tile::FIRST_KNIGHT | Tile::SECOND_KNIGHT => 3,
        Tile::FIRST_ROOK | Tile::SECOND_ROOK => 5,
        Tile::FIRST_KING | Tile::SECOND_KING => 0,
        Tile::FIRST_QUEEN | Tile::SECOND_QUEEN => 9,
        Tile::EMPTY => {
            panic!("Cannot generate material for an empty tile")
        }
        _ => {
            panic!("This should never happen - piece is not a valid recognised chesspiece")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_is_worth_one() {
        let mut state = GameState::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = MaterialHeuristic {}.evaluate(&state, &cache);

        assert_eq!(1, result);
    }

    #[test]
    fn given_multiple_pawns_count_all() {
        let mut state = GameState::from_fen("8/8/8/3PP3/8/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = MaterialHeuristic {}.evaluate(&state, &cache);

        assert_eq!(2, result);
    }

    #[test]
    fn knight_and_bishop_worth_three_each() {
        let mut state = GameState::from_fen("8/8/8/3NB3/8/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = MaterialHeuristic {}.evaluate(&state, &cache);

        assert_eq!(6, result);
    }

    #[test]
    fn rook_is_worth_five() {
        let mut state = GameState::from_fen("8/8/8/3R4/8/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = MaterialHeuristic {}.evaluate(&state, &cache);

        assert_eq!(5, result);
    }

    #[test]
    fn queen_is_worth_nine() {
        let mut state = GameState::from_fen("8/8/8/3Q4/8/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = MaterialHeuristic {}.evaluate(&state, &cache);

        assert_eq!(9, result);
    }

    #[test]
    fn minus_enemy_material_from_own_material() {
        let mut state = GameState::from_fen("8/8/8/3Qq3/8/8/8/8 w - - 0 1");

        let cache = HeuristicsCache::from(&mut state);
        let result = MaterialHeuristic {}.evaluate(&state, &cache);

        assert_eq!(0, result);
    }
}
