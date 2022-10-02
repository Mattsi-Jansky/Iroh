use crate::heuristics::Heuristic;
use crate::state::GameState;

struct MaterialHeuristic {}

impl Heuristic for MaterialHeuristic {
    fn evaluate(&self, state: GameState, is_first_player: bool) -> u32 {
        state.board.get_all_pieces_belonging_to_player(is_first_player)
            .len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_is_worth_one() {
        let state = GameState::from_fen("8/8/8/3P4/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(state, true);

        assert_eq!(1, result);
    }

    #[test]
    fn given_multiple_pawns_count_all() {
        let state = GameState::from_fen("8/8/8/3PP3/8/8/8/8 w - - 0 1");

        let result = MaterialHeuristic{}.evaluate(state, true);

        assert_eq!(2, result);
    }
}
