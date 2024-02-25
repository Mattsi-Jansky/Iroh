use crate::state::GameState;

pub enum GameStatus {
    Ongoing,
    Win,
    Draw
}

pub fn determine_status(state: &GameState) -> GameStatus {
    if state.possible_moves.is_empty() {
        if state.is_check(state.is_first_player_turn) {
            GameStatus::Win
        } else {
            GameStatus::Draw
        }
    } else {
        let mut is_first_player_turn = !state.is_first_player_turn;
        let mut first_player_sans = vec![];
        let mut second_player_sans = vec![];
        for san in state.sans.iter().rev() {
            if is_first_player_turn {
                first_player_sans.push(san);
            } else {
                second_player_sans.push(san);
            }
            is_first_player_turn = !is_first_player_turn;
        }

        if (!state.is_first_player_turn && state.is_fivefold_repetition(first_player_sans))
            || state.is_fivefold_repetition(second_player_sans)
            || state.turn_number - state.captured_pieces.last_capture_turn >= 75
        {
            GameStatus::Draw
        } else {
            GameStatus::Ongoing
        }
    }
}