use crate::heuristics::Heuristics;
use evaluation::Evaluation;
use possible_move::PossibleMove;
use std::collections::BinaryHeap;
use crate::state::GameState;
use crate::state::status::{determine_status_inner, GameStatus};

pub mod evaluation;
mod possible_move;

const MAX_DEPTH: u8 = 2;

pub fn search(game: &mut GameState) -> Evaluation {
    let heuristics = Heuristics::new();
    let mut results: BinaryHeap<PossibleMove> = BinaryHeap::new();
    let is_first_player = game.is_first_player_turn;

    for possible_move in game.possible_moves.iter() {
        let mut move_result = game.make_move(possible_move).unwrap();
        #[cfg(debug_assertions)]
        println!("Possible move START: {possible_move}");
        let value = minmax(
            &mut move_result,
            0,
            !is_first_player,
            &heuristics,
            i32::MIN,
            i32::MAX,
        );
        #[cfg(debug_assertions)]
        println!("Possible move OUTCOME: {possible_move}, {value}");

        results.push(PossibleMove {
            value,
            possible_move,
            is_maximising: is_first_player,
        });
    }

    Evaluation {
        best_move: results.pop().unwrap().possible_move.generate_san(),
    }
}

fn minmax(
    game_state: &mut GameState,
    depth: u8,
    is_maximising: bool,
    heuristics: &Heuristics,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    let status = determine_status_inner(game_state);
    let is_ongoing = !matches!(status, GameStatus::Ongoing);
    if depth == MAX_DEPTH || is_ongoing {
        heuristics.evaluate(game_state)
    } else {
        let mut best_value = if is_maximising { i32::MIN } else { i32::MAX };
        for possible_move in game_state.possible_moves.iter() {
            let mut move_result = game_state.make_move(possible_move).unwrap();
            let value = minmax(
                &mut move_result,
                depth + 1,
                !is_maximising,
                heuristics,
                alpha,
                beta,
            );
            if (is_maximising && value > best_value) || (!is_maximising && value < best_value) {
                best_value = value;
            }

            if is_maximising {
                if value >= beta {
                    break;
                }
                alpha = i32::max(value, alpha);
            } else {
                if value <= alpha {
                    break;
                }
                beta = i32::min(value, beta);
            }
        }
        best_value
    }
}
