use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap};
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

const DEPTH: usize = 1;
const BREADTH: usize = 5;

pub fn search(state: &GameState) -> Evaluation {
    let heuristics = Heuristics::new();
    let mut results: BinaryHeap<Node> = BinaryHeap::new();

    for possible_move in state.possible_moves.iter() {
        let move_result = state.make_move(&possible_move);
        let value = heuristics.evaluate(&move_result.unwrap());
        #[cfg(debug_assertions)]
        println!("Possible move: {possible_move}, {value}");

        results.push(Node {value, possible_move});
    }

    results = top_five(results);
    println!("Result top: {results:?}");

    Evaluation { best_move: results.pop().unwrap().possible_move.generate_san() }
}

fn top_five(mut input: BinaryHeap<Node>) -> BinaryHeap<Node> {
    let mut result: BinaryHeap<Node> = BinaryHeap::new();
    for i in 0..BREADTH {
        result.push(input.pop().unwrap());
    }
    result
}
