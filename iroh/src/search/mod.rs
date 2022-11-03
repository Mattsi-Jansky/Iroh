use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap};
use crate::game::Game;
use crate::heuristics::Heuristics;
use crate::moves::Move;
use crate::state::GameState;

pub struct Evaluation {
    pub best_move: String
}

#[derive(Debug)]
pub struct Node<'a> {
    value: i32,
    possible_move: &'a Move
}

impl<'a> PartialEq<Self> for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<'a> Eq for Node<'a> {}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }

    fn lt(&self, other: &Self) -> bool {
        self.value.lt(&other.value)
    }

    fn le(&self, other: &Self) -> bool {
        self.value.le(&other.value)
    }

    fn gt(&self, other: &Self) -> bool {
        self.value.gt(&other.value)
    }

    fn ge(&self, other: &Self) -> bool {
        self.value.ge(&other.value)
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

const MAX_DEPTH: u8 = 2;

pub fn search(game: &Game) -> Evaluation {
    let heuristics = Heuristics::new();
    let mut results: BinaryHeap<Node> = BinaryHeap::new();
    let state = game.unwrap();
    let is_first_player = state.is_first_player_turn;

    for possible_move in state.possible_moves.iter() {
        let move_result = state.make_move(&possible_move);
        let value = minmax(&move_result, 0, !is_first_player, &heuristics, i32::MIN, i32::MAX);
        #[cfg(debug_assertions)]
        println!("Possible move: {possible_move}, {value}");

        results.push(Node {value, possible_move});
    }

    Evaluation { best_move: results.pop().unwrap().possible_move.generate_san() }
}

fn minmax(game:&Game, depth: u8, is_maximising: bool, heuristics: &Heuristics, mut alpha: i32, mut beta: i32) -> i32 {
    let is_ongoing = !matches!(game, Game::Ongoing {..});
    let state = &game.unwrap();
    if depth == MAX_DEPTH || is_ongoing {
        if matches!(game, Game::Win {..}) {
            if is_maximising { i32::MIN } else {i32::MAX}
        } else { heuristics.evaluate(state) }
    } else {
        let mut best_value = if is_maximising { i32::MIN } else { i32::MAX };
        for possible_move in state.possible_moves.iter() {
            let move_result = state.make_move(&possible_move);
            let value = minmax(&move_result, depth + 1, !is_maximising, heuristics, alpha, beta);
            if is_maximising {
                alpha = if value > alpha { value } else { alpha };
            }else {
                beta = if value < beta { value } else { beta };
            }
            if beta <= alpha { break; }
            if (is_maximising && value > best_value)
                || (!is_maximising && value < best_value) {
                best_value = value;
            }
        }
        best_value
    }
}
