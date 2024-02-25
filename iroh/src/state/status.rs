use crate::game::Game;
use crate::state::GameState;

pub fn determine_status_or_illegal_move(game: &Game, state: Option<GameState>) -> Game {
    if let Some(state) = state {
        determine_status(state)
    } else {
        Game::IllegalMove { state: game.unwrap().clone() }
    }
}

pub fn determine_status(state: GameState) -> Game {
    if state.possible_moves.is_empty() {
        if state.is_check(state.is_first_player_turn) {
            Game::Win {
                is_first_player_win: !state.is_first_player_turn(),
                state,
            }
        } else {
            Game::Draw { state }
        }
    } else {
        let mut is_first_player_turn = !state.is_first_player_turn;
        let mut first_player_sans = vec![];
        let mut second_player_sans = vec![];
        for san in state.sans.clone().into_iter().rev() {
            if is_first_player_turn {
                first_player_sans.push(san);
            } else {
                second_player_sans.push(san);
            }
            is_first_player_turn = !is_first_player_turn;
        }

        if (!state.is_first_player_turn && state.is_fivefold_repetition(&first_player_sans))
            || state.is_fivefold_repetition(&second_player_sans)
            || state.turn_number - state.captured_pieces.last_capture_turn >= 75
        {
            Game::Draw { state }
        } else {
            Game::Ongoing { state }
        }
    }
}