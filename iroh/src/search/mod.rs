use crate::heuristics::Heuristics;
use evaluation::Evaluation;
use possible_move::PossibleMove;
use std::collections::BinaryHeap;
use crate::moves::resolve_move::{undo_move, undo_turn_including_turn_number};
use crate::state::GameState;
use crate::state::status::{determine_status, GameStatus};

pub mod evaluation;
mod possible_move;

const MAX_DEPTH: u8 = 2;

pub fn search(game_state: &mut GameState) -> Evaluation {
    let heuristics = Heuristics::new();
    let mut results: BinaryHeap<PossibleMove> = BinaryHeap::new();
    let is_first_player = game_state.is_first_player_turn;

    let possible_moves = game_state.pop_possible_moves();
    for possible_move in possible_moves.iter() {
        #[cfg(debug_assertions)]
        println!("Possible move START: {possible_move}");
        let memento = game_state.make_legal_move_mut_no_check(possible_move).unwrap();
        let value = minmax(
            game_state,
            0,
            !is_first_player,
            &heuristics,
            i32::MIN,
            i32::MAX,
        );
        undo_turn_including_turn_number(memento, game_state);
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
    let status = determine_status(game_state);
    let is_ongoing = matches!(status, GameStatus::Ongoing);
    if depth == MAX_DEPTH || !is_ongoing {
        heuristics.evaluate(game_state)
    } else {
        let mut best_value = if is_maximising { i32::MIN } else { i32::MAX };
        let possible_moves = game_state.pop_possible_moves();
        for possible_move in possible_moves {

            let memento = game_state.make_legal_move_mut_no_check(&possible_move).unwrap();
            let value = minmax(
                game_state,
                depth + 1,
                !is_maximising,
                heuristics,
                alpha,
                beta,
            );
            undo_turn_including_turn_number(memento, game_state);
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
