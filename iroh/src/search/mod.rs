use crate::heuristics::Heuristics;
use crate::state::GameState;

pub struct Evaluation {
    pub best_move: String
}

pub fn search(state: &GameState) -> Evaluation {
    let heuristics = Heuristics::new();
    let mut best = (i32::MIN, String::new());

    for possible_move in state.possible_moves.iter().map(|m| m.generate_san()) {
        let move_result = state.make_move(&possible_move);
        let value = heuristics.evaluate(&move_result.unwrap(), state.is_first_player_turn);
        if value > best.0 {
            best = (value, possible_move);
        }
    }

    Evaluation { best_move: best.1 }
}
